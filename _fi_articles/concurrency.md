---
title: Rinnakkaisuus ja asynkroninen ohjelmointi
lang: fi-FI
translation_key: concurrency
---

Tässä moduulissa tarkastellaan rinnakkaista prosessointia **säikeiden** avulla.
Verkkopalvelimella säikeet mahdollistavat tehokkaamman rinnakkaisen suorituksen
nykyaikaisissa tietokoneissa, joissa on useita prosessoriytimiä, jolloin
käyttöjärjestelmä voi skeduloida eri säikeet suoritettaviksi samanaikaisesti eri
ytimillä. Asiakasohjelmassa säikeitä voidaan käyttää verkkokommunikaation
toteuttamiseen siten että käyttäjä voi samalla käyttää graafista käyttöliittymää
reaaliaikaisesti.

Rinnakkaisen suorituksen tukemiseksi Rustin omistajuussääntöjen puitteissa
tutustutaan **jaettuihin viitelaskettuihin osoittimiin** sekä
**Mutex-lukkoihin**, jotka suojaavat tiedon eheyttä silloin kun rinnakkaiset
säikeet käyttävät samaa tilatietoa.

Lopuksi tarkastellaan **asynkronista ohjelmointia Tokion-kirjaston** avulla.
Siinä rinnakkaiset toiminnot, esimerkiksi useiden asiakkaiden palveleminen,
jaetaan kevyisiin tehtäviin, joita käyttäjätasolla toimiva Tokio-ajoympäristö
suorittaa ja skeduloi käyttöjärjestelmän skeduloimien säikeiden sijaan.

<div class="objectives-frame" markdown="1">

**Tämän moduulin tavoitteet:**

- **Ymmärrät älykkäiden osoittimien periaatteen** dynaamisten tietotyyppien ja
  jaetun omistajuuden yhteydessä sekä osaat **käyttää niitä Rustissa**.

- Osaat **käyttää säikeitä sovelluksen rinnakkaisen toiminnan hallintaan** niin,
  että palvelin ja asiakasohjelma toimivat sujuvasti ja tehokkaasti.

- **Ymmärrät, miten Mutex-lukot toimivat ja miten niitä käytetään yhdessä
  älykkäiden osoittimien kanssa** suojaamaan jaettua dataa, kun useat säikeet
  käsittelevät sitä samanaikaisesti.

- Opit **soveltamaan asynkronista ohjelmointimallia** erityisesti käyttämällä
  Tokio-kirjastoa asynkronisena ajoympäristönä.

</div>

## Älykkäät osoittimet muistinhallinnassa

Tähän asti olemme käyttäneet paikallisia muuttujia ja arvoja, jotka varataan
ohjelman suorituksen aikana tyypillisesti pinosta. Poikkeuksena ovat
`String`-tyyppi ja kokoelmatyypit, jotka käyttävät kekomuistia. Joskus
paikallinen muistinvaraus aiheuttaa vaikeuksia: jos ohjelmassa on asynkronista
logiikkaa tai rinnakkain suoritettavia säikeitä, omistajuuden hallinta
perusmekanismeilla voi olla vaikeaa tai jopa mahdotonta. Toisaalta joissakin
tilanteissa tarvittavan muistin määrää ei tiedetä käännösaikana, esimerkiksi
käsiteltäessä olioita, joiden tyyppi määräytyy dynaamisesti. Tällaisissa
tapauksissa tarvitaan jotakin Rustin älykkäistä osoitintyypeistä, jotta arvo
voidaan sijoittaa kekomuistiin ja sen elinkaarta hallita älykkäiden osoittimien
tarjoamilla mekanismeilla.

### Muistin varaaminen keosta

Kenties suoraviivaisin tapa varata muistia keosta on käyttää `Box<T>`-tyyppiä,
joka sijoittaa sisältämänsä arvon kekomuistiin. Muisti vapautetaan normaalisti,
kun `Box` poistuu näkyvyysalueeltaan. Seuraava yksinkertainen esimerkki
`Box`-arvon luomisesta ja käytöstä on kopioitu [Rust-kirjan luvusta
15.1](https://doc.rust-lang.org/stable/book/ch15-01-box.html), jossa
`Box`-tyyppiä käsitellään tarkemmin.

```rust
fn main() {
    let b = Box::new(5); // tyyppi: i32
    println!("b = {b}");
}
```

![Kekomuistiin Box:in avulla tallennettu kokonaisluku](/images/concurrency-boxbasic.svg){: width="90%" .center-img }

`Box`-tyyppi on hyödyllinen, kun varattavan tiedon tarkkaa kokoa ei tiedetä
käännösaikana. Sen avulla Rustissa voidaan toteuttaa olio-ohjelmoinnin kaltaista
polymorfismia käyttämällä **piirteitä** (trait), jotka määrittelevät eri
tietotyyppien toteutettavaksi yhteisen rajapinnan. Esimerkiksi
`TcpStream`-tyyppiä toteuttaa `Read`- ja `Write`-piirteet, jotka määrittävät
yhteiset operaatiot virtamuotoisen tiedon lukemiseen ja kirjoittamiseen.

Jos toteuttaisimme valkotaulusovellusta, voisimme määritellä
`WhiteboardObject`-piirteen, joka sisältää `draw()`-metodin. Tyypit `Text`,
`Rectangle` ja `Line` voisivat toteuttaa tämän piirteen ja sen `draw()`-metodin.
`Box` tyypin avulla voimme määritellä että muuttuja voi sisältää dynaamisesti
minkä tahansa annetun piirteen toteuttavan tietotyypin. Esimerkiksi seuraavassa
esimerkissä käytetään vektoria, joka voi sisältää minkä tahansa
`Box`-osoittimeen sisälletyn `WhiteboardObject`-alkion:

```rust
trait WhiteboardObject {
    fn draw(&self);
}

struct Text(String);
struct Rectangle { width: u32, height: u32 }
struct Line { length: u32 }

impl WhiteboardObject for Text {
    fn draw(&self) { println!("Text: {}", self.0); }
}

impl WhiteboardObject for Rectangle {
    fn draw(&self) { println!("Rectangle: {} x {}", self.width, self.height); }
}

impl WhiteboardObject for Line {
    fn draw(&self) { println!("Line: length {}", self.length); }
}

fn main() {
    let objects: Vec<Box<dyn WhiteboardObject>> = vec![
        Box::new(Text(String::from("Hello!"))),
        Box::new(Rectangle { width: 100, height: 50 }),
        Box::new(Line { length: 80 }),
    ];

    for object in &objects {
        object.draw();
    }
}
```

Vaikka vektorin alkioiden tyypiksi on määritelty `Box<dyn WhiteboardObject>`,
kekomuistiin tallennetut varsinaiset oliot ovat tyypiltään `Text`, `Rectangle`
tai `Line`, ja niiden viemä tila muistissa vaihtelee. Avainsana `dyn`
mahdollistaa dynaamisen metodinvalinnan: `draw()`-kutsu valitsee suoritusaikana
kyseisen olion tyypin mukaisen toteutuksen. Yksinkertaisuuden vuoksi esimerkki
tulostaa vain tekstiä varsinaisen grafiikan piirtämisen sijaan.

![Dynaamisia alkioita Box-osoittimien avulla sisältävän vektorin muistirakenne](/images/concurrency-box.svg){: width="100%" .center-img }

[Rust-kirjan luvussa 18](https://doc.rust-lang.org/stable/book/ch18-00-oop.html)
käsitellään Rustin olio-ohjelmointiominaisuuksia tarkemmin.

### Viitelaskettu osoitin: Rc

Joissakin tilanteissa Rustin omistajuussäännöt voivat olla rajoittavia, ja
omistajuus on jaettava usean käyttäjän kesken. Tätä tarvitaan usein erityisesti
asynkronisessa ohjelmoinnissa tai käytettäessä rinnakkaisuutta ja säikeitä,
jolloin tapahtumien ajoitus ja jaetun tiedon käsittely ei aina ole ennalta
määritelty. Jos jaamme tietoa monisäikeisessä ohjelmassa, mistä tiedämme,
milloin sen käyttämä muisti voidaan vapauttaa? Vaikka yhden säikeen suoritus
olisi tullut näkyvyysalueen loppuun ja se olisi valmis vapauttamaan tiedon, muut
säikeet saattavat yhä tarvita sitä.

`Rc<T>`-tyyppi tallentaa tietonsa kekomuistiin, mutta toisin kuin `Box`, se
mahdollistaa kekomuistiin osoittavan viitteen jakamisen usean omistajan kesken.
Uudelle omistajalle voidaan antaa pääsy tietoon `clone()`-funktiolla siten, että
myös aiemmat omistajat säilyttävät omistajuutensa. Tietoon osoittavan viitteen
lisäksi `Rc` ylläpitää **viitelaskuria**, joka kertoo tietoon pääsevien
omistajien lukumäärän (tästä tulee myös tyypin nimi, "refrence count"). Jokainen
`clone()`-kutsu kasvattaa viitelaskuria, ja laskuri pienenee, kun `Rc`-muuttujat
poistuvat näkyvyysalueeltaan. Kun viimeinenkin omistaja poistuu, viitelaskuri
laskee nollaan ja tiedon käyttämä muisti voidaan vapauttaa.

Esimerkiksi kaksi muuttujaa voi jakaa saman merkkijonon omistajuuden:

```rust
use std::rc::Rc;

fn main() {
    let message = Rc::new(String::from("Hello!"));
    println!("Owners: {}", Rc::strong_count(&message)); // 1

    {
        let shared_message = Rc::clone(&message);
        println!("message: {}   shared: {}", message, shared_message);
        println!("Owners: {}", Rc::strong_count(&message)); // 2
    } // shared_message tuhotaan tässä

    println!("Owners: {}", Rc::strong_count(&message)); // 1
} // Viimeinen omistaja tuhotaan, jolloin merkkijonon muisti vapautetaan
```

`Rc::clone(&message)` luo samalle kekomuistin varaukselle uuden omistajan, mutta
se ei kopioi itse merkkijonoa. Alkuperäistä `message`-muuttujaa voi käyttää
edelleen sen jälkeen, kun `shared_message` poistuu näkyvyysalueeltaan.

![Jaettu merkkijono](/images/concurrency-rc.svg){: width="100%" .center-img }

Lisätietoa `Rc`-tyypistä on [Rust-kirjan luvussa
15.4](https://doc.rust-lang.org/stable/book/ch15-04-rc.html).

### Atominen viitelaskenta: Arc

`Rc`-tyyppi ei ole turvallinen rinnakkaisessa käytössä, joten sitä voi käyttää
vain yhden säikeen sisällä. Kun omistajuus täytyy jakaa usean säikeen kesken,
tarvitaan **atomista viitelaskentaa**, jonka tarjoaa `Arc<T>`-tyyppi. Atomista
viitelaskentaa lukuun ottamatta `Arc` toimii samalla tavalla kuin `Rc`.

## Säikeet

Nykyaikaiset käyttöjärjestelmät järjestävät sovellukset ja palvelut
**prosesseiksi**, joille käyttöjärjestelmä varaa eristetyn (virtuaalisen)
muistin ja muita resursseja. Prosesseja suoritetaan yhtäaikaisesti
käyttöjärjestelmän skeduloijan ohjaamana. Käyttöjärjestelmän tarjoaman vahvan
eristyksen ansiosta ohjelmat voidaan suunnitella ja toteuttaa turvallisesti
ilman, että ne häiritsevät toisiaan (ainakin niin kauan kun vapaita
järjestelmäresursseja riittää), ellei käytössä ole jotakin prosessien välistä
viestintätapaa, kuten pistokkeita.

Prosessin sisällä suoritus voidaan jakaa erillisiin suoritusyksiköihin,
**säikeisiin**. Saman prosessin säikeet jakavat samat resurssit, kuten
kekomuistin ja ohjelmakoodin, mutta käyttöjärjestelmä suorittaa ja ajoittaa
niitä erikseen. Eri säikeitä voidaan suorittaa järjestelmän eri
suoritinytimillä, joten säikeillä voidaan saavuttaa aidosti rinnakkainen
suoritus ja parantaa tehokkuutta esimerkiksi kuormitetuilla palvelimilla.
Jokaisella säikeellä on oma kutsupinonsa ja omat rekisterinsä, joissa
säilytetään esimerkiksi paikallisia muuttujia ja suorituksen tilaa.

Säikeet ovat yksi tapa toteuttaa verkkopalvelin, jonka täytyy palvella useita
asiakkaita samanaikaisesti ilman viiveitä. Jos jokaiselle asiakkaalle varataan
oma säie, blokkaavat pistokkeiden I/O-operaatiot (esimerkiksi `read()`)
pysäyttävät vain kyseisen säikeen vaikuttamatta muihin. Joissakin tapauksissa
palvelimen toteutuslogiikka on tällöin yksinkertaisempi kuin multipleksoitua
I/O:ta käyttävän, poll-kutsuun perustuvan palvelimen käsittelysilmukka — ainakin
siihen asti, kunnes säikeiden välillä täytyy jakaa tietoa.

Seuraava palvelin käynnistää erillisen säikeen jokaista hyväksyttyä yhteyttä
varten. Pääsäie voi näin jatkaa uusien asiakkaiden odottamista, vaikka
olemassa olevat pistokkeet odottaisivat dataa blokkaavassa kutsussa.

Uusi säie käynnistetään `spawn()`-funktiolla. Sen parametri poikkeaa aiemmin
nähdyistä: parametrina annetaan toinen funktio, joka suoritetaan uudessa
säikeessä. Nimetyn funktion sijaan käytetään usein **sulkeumaa**, kuten tässä
esimerkissä. Sulkeuma on nimetön funktio, joka määritellään suoraan
`spawn()`-kutsun yhteydessä. Se voi kaapata muuttujia ympäröivästä
näkyvyysalueestaan. Tässä kaapataan `stream`-muuttuja, joka sisältää juuri
hyväksytyn asiakkaan pistokkeen. Säikeiden yhteydessä käytetään tässä
`move`-avainsanaa. Se siirtää kaapattujen muuttujien eli tässä
`stream`-muuttujan omistajuuden sulkeuman sisälle ja siten uuden säikeen
käyttöön. Pääsäie ei voi enää käyttää muuttujaa, ja kun käynnistetty säie
päättyy, `stream`-yhteys vapautetaan. Sulkeumia käsitellään tarkemmin
[Rust-kirjan luvussa
13.1](https://doc.rust-lang.org/stable/book/ch13-01-closures.html).

```rust
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

// Lue yhden asiakkaan dataa ja kaiuta se takaisin, kunnes asiakas katkaisee yhteyden.
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }

        stream.write_all(&buffer[..bytes_read])?;
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:2000")?;
    println!("Listening on 0.0.0.0:2000");

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                let address = stream.peer_addr()?;
                println!("Accepted connection from {address}");

                thread::spawn(move || {
                    if let Err(error) = handle_client(stream) {
                        eprintln!("Connection to {address} failed: {error}");
                    }
                });
            }
            Err(error) => eprintln!("Failed to accept connection: {error}"),
        }
    }

    Ok(())
}
```

[Rust-kirjan luku
16](https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html) käsittelee
rinnakkaisuutta ja säikeitä tarkemmin ja tarjoaa lisää esimerkkejä.
Esimerkkihakemistossa on myös
**[threaded-server](https://github.com/PasiSa/pronets/blob/main/examples/threaded-server/src/main.rs)**,
joka esittelee erillisen säikeen luomista kullekin uudelle asiakkaalle.

Koska käyttöjärjestelmä ajoittaa säikeet ja niitä voidaan suorittaa
samanaikaisesti useilla suorittimilla, eri säikeiden tapahtumien keskinäinen
järjestys ei ole ennalta määrätty. Säikeiden täytyy usein myös käsitellä
yhteistä tilaa tai välittää tietoa toisilleen. Jaetun tiedon käsittelyn täytyy
olla atomista, jotta tietorakenteet pysyvät johdonmukaisessa tilassa silloinkin,
kun niitä käsittelee mahdollisesti suuri määrä säikeitä. Rust pyrkii
suojaamaan tietoa samanaikaiselta käytöltä ja tarjoaa tähän muutamia työkaluja,
joita käsitellään seuraavaksi.

## Mutex-lukot rinnakkaisessa tiedon käsittelyssä

Kuten edellä kerrottiin, tiedon omistajuus voidaan jakaa viitelaskettujen,
keosta varattujen muuttujien avulla. Tieto säilyy keossa niin kauan kuin
muuttujaan on vähintään yksi viite. Kun viimeinen viite poistuu näkyvyysalueelta
ja viitelaskuri laskee nollaan, tieto ja sen käyttämä muisti vapautetaan.
`Arc<T>`-tyypin avulla voimme jakaa yhteistä tietoa tällä tavalla myös useiden
säikeiden kesken.

Pelkkä `Arc` ei kuitenkaan yleensä riitä, kun tietoa käsitellään useissa
säikeissä. Meidän täytyy varmistaa, että vain yksi säie kerrallaan pääsee
käsittelemään muutettavaa tietoa. Myös Rustin kääntäjä pyrkii auttamaan
ohjelmoijaa vaatimalla tätä. Siksi `Arc`-osoittimen osoittama tieto voidaan
kääriä `Mutex`-tyypin (**mutual exclusion**, keskinäinen poissulkeminen) sisään
suojaamaan sitä samanaikaiselta käytöltä. Mutex ylläpitää lukkoa, joka valvoo
tiedon käyttöä eikä päästä muita käsittelemään tietoa ennen kuin edellinen
käyttäjä on lopettanut.

`Mutex` suojaa sisältämänsä tiedon käyttöä `lock()`-metodilla. Jos lukko on
vapaa, metodi palauttaa `MutexGuard`-arvon, jonka kautta tietoa voi lukea ja
muuttaa. Jos joku on jo varannut lukon, `lock()`-kutsu blokkaa, kunnes lukko
vapautuu. Rustille tyypilliseen tapaan lukko vapautuu, kun `MutexGuard`-muuttuja
poistuu näkyvyysalueelta. Seuraava kuva havainnollistaa muistin rakennetta, kun
kaksi säiettä jakaa yhteisen _MyState_-tilan `Arc<Mutex<MyState>>`-muuttujien
avulla. Vihreä alue kuvaa **Arc**-tyyppiä, harmaa alue **Mutex**-tyyppiä ja
violetti alue varsinaista jaettua tietoa. Tyyppien todelliset toteutukset voivat
poiketa kuvasta: kuvan nimet ovat havainnollistavia ja yksinkertaistettuja.

![Mutex-lukolla suojattu viitelaskettu jaettu tieto](/images/concurrency-mutex.svg){: width="100%" .center-img }

Seuraava esimerkki on hieman muokattu versio [Rust-kirjan luvun
16.3](https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html)
esimerkistä.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Jaettu tila: mutex-lukolla suojattu laskuri.
    let counter = Arc::new(Mutex::new(0));

    // Vektori säiekahvoille.
    let mut handles = Vec::new();

    for _ in 0..4 {
        // Kloonataan Arc, jotta omistajuus voidaan jakaa säikeiden kesken.
        // Arc ylläpitää samaan tietoon osoittavien viitteiden lukumäärää.
        let clonedcounter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Lukitaan mutex ennen jaetun tiedon käsittelyä.
            // lock().unwrap() palauttaa MutexGuard-arvon, jonka `value` säilyttää.
            // Suojattuun laskuriin päästään käsiksi *-operaattorilla.
            let mut value = clonedcounter.lock().unwrap();
            *value += 1;
            // Lukko vapautuu automaattisesti, kun `value` poistuu näkyvyysalueelta.
        });

        handles.push(handle);
    }

    // Odotetaan kaikkien säikeiden päättymistä: join() blokkaa nykyisen säikeen,
    // kunnes kahvan edustama säie päättyy. Se palauttaa Err-arvon, jos säie
    // päättyi paniikkiin.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Laskurin lopullinen arvo: {}", *counter.lock().unwrap());
}
```

Mutexien ja lukkojen käyttöön liittyy lukkiutumisen (deadlock) riski.
Lukkiutumistilanteiden debugaaminen voi olla vaikeaa ohjelmien
monimutkaistuessa. Esimerkiksi yksi säie voi pitää lukkoa hallussaan
odottaessaan tapahtumaa, jonka toinen säie voi tuottaa vasta saatuaan saman
lukon. Kumpikaan säie ei pääse etenemään, joten molemmat odottavat loputtomasti.
Riskin pienentämiseksi kriittiset alueet kannattaa pitää lyhyinä ja blokkaavia
operaatioita välttää lukon ollessa hallussa. Jos tarvitaan useita lukkoja,
ohjelman tulisi varata ne samassa järjestyksessä kaikissa säikeissä.

## Viestikanavat säikeiden väliseen viestintään

Toinen tapa jakaa tietoa säikeiden välillä on **viestinvälitys**. Rustin
standardikirjasto tarjoaa `mpsc`-viestikanavia ("multiple producers, single
consumer", eli useita lähettäjiä ja yksi vastaanottaja). Kun kanava luodaan,
saadaan erilliset kahvat viestien lähettämiseen ja vastaanottamiseen. Kun uusi
säie käynnistetään, toinen kahvoista voidaan siirtää sille ja toinen jättää
pääsäikeeseen. Kanavan kautta voidaan välittää erityyppisiä arvoja. Kun
lähettäjä lähettää arvon `send()`-kutsulla, myös arvon omistajuus siirtyy
vastaanottajalle, joka ottaa arvon vastaan `recv()`-kutsulla. Siksi lähettäjä ei
voi enää käyttää arvoa lähettämisen jälkeen. [Rust-kirjan luku
16.2](https://doc.rust-lang.org/stable/book/ch16-02-message-passing.html)
käsittelee viestinvälitystä tarkemmin.

Viestinvälitystä voidaan käyttää esimerkiksi vuorovaikutteisen asiakasohjelman
toteutuksessa. Pääsäie voisi vastata käyttöliittymästä ja sujuvasta
vuorovaikutuksesta käyttäjän kanssa sekä esimerkiksi yksinkertaisen pelin
reaaliaikaisista animaatioista. Toinen säie voitaisiin käynnistää hoitamaan
viestintää palvelimen kanssa. Jos viestintäsäie blokkaa odottaessaan uutta
dataa, se ei haittaa pääsäiettä eikä sen reaaliaikaista vuorovaikutusta.
Viestikanavan avulla voidaan välittää pistokkeesta saadut tilapäivitykset
pääsäikeelle, joka tekee tarvittavat muutokset käyttöliittymään.

![Säikeiden yhdistäminen kanavalla](/images/concurrency-channel.svg){: width="90%" .center-img }

Alla oleva yksinkertaistettu esimerkki simuloi palvelimen päivitysten
odottamista `sleep()`-kutsulla. Todellisessa asiakasohjelmassa viestintäsäie
lukisi pistokkeesta. Pääsäie tarkistaa päivitykset `try_recv()`-kutsulla
blokkaamatta, joten se voi jatkaa käyttöliittymän päivittämistä myös silloin,
kun viestejä ei ole saatavilla.

```rust
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    // Siirretään kanavan lähettävä pää uuteen säikeeseen
    let communication = thread::spawn(move || {
        for status in ["Yhteys muodostettu", "Uusi viesti", "Palvelinyhteys katkennut"] {
            // Simuloidaan blokkaavaa lukua pistokkeesta viestintäsäikeessä.
            thread::sleep(Duration::from_millis(500));
            sender.send(String::from(status)).unwrap();
        }
        // Lähettäjän pudottaminen katkaisee kanavan yhteyden.
    });

    loop {
        match receiver.try_recv() {
            Ok(status) => println!("Päivitetään käyttöliittymä: {status}"),
            Err(TryRecvError::Empty) => {} // Ei vielä päivitystä; jatketaan suoritusta.
            Err(TryRecvError::Disconnected) => break,
        }

        // Tässä käsitellään käyttäjän syöte ja piirretään seuraava animaatioruutu.
        // Simuloidaan ruutujen välistä taukoa, jotta silmukka ei pyöri jatkuvasti.
        thread::sleep(Duration::from_millis(16));
    }

    // Odotetaan viestintäsäikeen päättymistä.
    communication.join().unwrap();
}
```

## Asynkroninen ohjelmointi

Säikeitä käytettäessä ohjelmoijan ei tarvitse huolehtia suorituksen ajoittamisesta
tai rinnakkaisuudesta, kunhan jaettun datan käsittely tapahtuu hallitusti.
Säikeiden runsas käyttö aiheuttaa kuitenkin käyttöjärjestelmälle lisätyötä, joka
voi käydä kalliiksi, kun samanaikaisia toimintoja on paljon, esimerkiksi lukuisia
asiakasistuntoja käsittelevällä palvelimella. Säikeiden etuna on kuitenkin se,
että ne voivat hyödyntää tehokkaasti järjestelmän useita suoritinytimiä.

Toinen lähestymistapa on asynkroninen ohjelmointimalli, jossa säikeiden
kaltaista samanaikaista suoritusta voidaan toteuttaa yhden säikeen sisällä. Tätä
varten sovellusohjelman rinnalla toimii asynkroninen ajoympäristö, joka skeduloi
tehtävien suorituksen yhden säikeen sisällä. Koska käyttöjärjestelmän ei
tarvitse skeduloida näitä tehtäviä, menetelmä on kevyempi ja tehokkaampi.
Käytännössä on usein parasta yhdistää molemmat lähestymistavat: käyttää usean
säikeen poolia hyödyntämään käytettävissä olevia suoritinytimiä ja soveltaa
kunkin säikeen sisällä myös asynkronista ohjelmointimallia.

Rustissa asynkroninen ohjelmointi perustuu **Future**-piirteen toteuttaviin
arvoihin eli futuureihin. Ne edustavat operaatioita, jotka voivat valmistua
myöhemmin, esimerkiksi kun operaation täytyy odottaa datan saapumista verkosta.
`async`-määreellä esitellyn funktion kutsuminen tai `async`-lohkon evaluointi
luo futuurin. Sen sisältämää koodia ei suoriteta ennen kuin futuuria pollataan,
eli sitä pyydetään etenemään. Asynkroninen ajoympäristö vastaa tästä
pollaamisesta.

`async`-funktion tai -lohkon sisällä `.await` odottaa futuurin valmistumista ja
tuottaa sen palauttaman arvon. Jos futuuri ei ole valmis, nykyisen tehtävän
suoritus voi keskeytyä, jolloin ajoympäristö voi suorittaa muita tehtäviä samassa
säikeessä. Jos futuuri on heti valmis, keskeytystä ei tarvita. Esimerkiksi
asynkroninen lukeminen pistokkeesta ja mutexin asynkroninen lukitseminen käyttävät
tätä mekanismia odottamiseen blokkaamatta säiettä. Pelkkä `.await`-ilmauksen
lisääminen ei muuta tavallista blokkaavaa operaatiota asynkroniseksi, vaan
operaation täytyy tarjota asynkroninen rajapinta.

Rust-kirjan [luvussa
17.1.](https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html)
käsitellään aihetta tarkemmin.

### Asynkroninen ajoympäristö ja blokkaamaton I/O Tokion avulla

Asynkronisten ohjelmien toteuttamiseen tarvitaan ohjelmointikielen tarjoamien
rakenteiden lisäksi asynkronisten tehtävien ajoympäristö. Tähän tarkoitukseen on
saatavilla useita skedulointikirjastoja, mutta tässä käsittelemme
**[Tokiota](https://crates.io/crates/tokio)**, joka on yksi käytetyimmistä.
Tokio tukee monisäikeisyyttä yhdessä asynkronisen ohjelmoinnin kanssa ja tarjoaa
asynkroniset versiot useista standardikirjaston funktioista.

Yksinkertainen Tokiolla toteutettu palvelin löytyy
[esimerkeistä](https://github.com/PasiSa/pronets/blob/main/examples/async-server/src/main.rs).
Tokiota käytettäessä `main()`-funktio määritellään asynkroniseksi ja sen
määrittelyn eteen lisätään `#[tokio::main]`-attribuutti. Rust-kääntäjä
käsittelee attribuutit ja tuottaa niiden perusteella koodia funktion ympärille.
Tässä tapauksessa attribuutti luo Tokion ajoympäristön, joka suorittaa
pääfunktion asynkronisen version. Ajoympäristö skeduloi asynkronisia tehtäviä
ja seuraa I/O-operaatioiden ja ajastimien tilaa, jotta tehtävien suoritusta
voidaan jatkaa, kun ne voivat edetä.

Tokion virallisella verkkosivustolla on hyvä
**[Tokio-opas](https://tokio.rs/tokio/tutorial)**. Oppaassa esitellään, mitä
käytännössä tapahtuu, kun `main()`-funktioon lisätään
`#[tokio::main]`-attribuutti. Seuraava pääfunktion toteutus:

```rust
#[tokio::main]
async fn main() {
    println!("hello");
}
```

muuntuu kääntäjän toimesta seuraavaan muotoon, jolla sen voisi myös korvata:

```rust
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello");
    })
}
```

`block_on()`-funktio suorittaa parametrina saamansa futuurin loppuun asti.
Jos suoritus kohtaa `.await`-ilmauksen, jonka odottama futuuri ei ole vielä
valmis, ajoympäristö voi jatkaa jonkin muun tehtävän suoritusta. Tokio käyttää
sisäisesti esimerkiksi **Mio**-kirjastoa valmiiden I/O-lähteiden pollaamiseen
tehtävien skeduloinnin yhteydessä.

Kun esimerkiksi operaatio `socket.read(&mut buf).await` ei voi valmistua heti,
asiakasta käsittelevän tehtävän suoritus keskeytyy ja Tokio voi suorittaa muita
tehtäviä. Kun pistoke on valmis luettavaksi, Tokio vuorontaa tehtävän jatkamaan
suoritustaan. Näin suuri määrä yhteyksiä voivat jakaa pienen (mutta optimaalisen
rinnakkaisuuden näkökulmasta tarvittavan) määrän työsäikeitä.

Tehtävän käynnistämisen rajapinta näyttää hyvin samankaltaiselta kuin säikeen
käynnistäminen:

```rust
task::spawn(process_client(socket, address));
```

Toiminta on kuitenkin erilaista. Uutta säiettä ei luoda, mikä vaatisi
käyttöjärjestelmän osallistumista. Sen sijaan Tokio luo kevyen tehtävän, ja
yhteyksiä hyväksyvä silmukka voi jatkaa seuraavan yhteyden odottamista.

Tokiota käytettäessä ohjelman täytyy käyttää **normaalien I/O-tietotyyppien
asynkronisia versioita**, kuten `TcpListener`- ja `TcpStream`-tyyppejä, jotta ne
osaisivat toimia yhteen Tokio-ajoympäristön kanssa. Siksi esimerkin alussa
käytetään tyyppejä `tokio::net::TcpListener` ja `tokio::net::TcpStream` Rustin
standardikirjaston versioiden sijaan.

Myös asynkronisissa tehtävissä **omistajuudella on edelleen merkitystä**.
Esimerkki siirtää pistokkeen omistajuuden uudelle tehtävälle, minkä jälkeen
pääsilmukka ei voi enää käyttää sitä. Eri tehtävien välillä jaettavaa dataa
varten saatetaan edelleen tarvita viitelaskettuja älykkäitä osoittimia. Koska
Tokion ajoympäristö voi jakaa tehtäviä useille säikeille, tarvitaan säikeiden
tavoin `Arc`-tyyppiä ja mutexeja suojaamaan samanaikaiselta käytöltä. Myös
`Mutex`-tyypistä on Tokiolle oma versio, `tokio::sync::Mutex`, jonka
`lock()`-funktio tukee `.await`-ilmausta. Jos lukkoa ei ole saatavilla, tehtävän
suoritus voi keskeytyä, ja Tokion ajoympäristö jatkaa sitä myöhemmin, kun lukko
on vapautunut.

### Asiakas/palvelin - testien kirjoittaminen Tokiolla

Kun toteutus käyttää Tokiota samanaikaisuuden hallintaan, Tokiota kannattaa
käyttää myös testeissä, joissa asiakas ja palvelin viestivät keskenään. Näin
voidaan toteuttaa kätevästi esimerkiksi integraatiotestejä.

Attribuutti
[`#[tokio::test]`](https://docs.rs/tokio/latest/tokio/attr.test.html) luo
Tokio-ajoympäristön asynkronista testiä varten. Sen avulla palvelinta voidaan
suorittaa erillisessä Tokio-tehtävässä samalla, kun itse testi toimii
asiakkaana. Seuraava esimerkkitesti tarkistaa, että TCP-palvelin lähettää
asiakkaan lähettämän datan takaisin sellaisenaan. Se voisi olla kuvitteellisen
ohjelman integraatiotesti.

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn server_echoes_client_data() {
    timeout(Duration::from_secs(5), async {
        // Sido pistoke ennen yhteyden muodostamista; portti 0 antaa käyttöjärjestelmän valita vapaan portin.
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = [0; 1024];
            loop {
                let count = socket.read(&mut buffer).await.unwrap();
                if count == 0 {
                    break; // Asiakas on lopettanut lähettämisen.
                }
                socket.write_all(&buffer[..count]).await.unwrap();
            }
        });

        let mut client = TcpStream::connect(address).await.unwrap();
        let message = b"Hello, Tokio!";
        client.write_all(message).await.unwrap();
        // Sulje vain lähetyssuunta, jotta kaiun vastaanottaminen on edelleen mahdollista.
        client.shutdown().await.unwrap();

        let mut response = Vec::new();
        client.read_to_end(&mut response).await.unwrap();
        server.await.unwrap();
        assert_eq!(response, message);
    })
    .await
    .expect("client/server test timed out");
}
```

`async move` -lohko siirtää kuuntelevan pistokkeen omistajuuden palvelintehtävälle.
Molemmat tehtävät voivat edetä testien oletusarvoisessa yksisäikeisessä
ajoympäristössä, koska verkon I/O-operaatioiden odottaminen luovuttaa
suoritusvuoron Tokiolle. Testeissä palvelin kannattaa sitoa porttiin 0, jolloin
käyttöjärjestelmä valitsee portin. Valitun portin voi tämän jälkeen selvittää
`local_addr()`-metodilla. Kiinteä portti saattaa olla jo jonkin muun ohjelman
käytössä, jolloin testi epäonnistuisi syystä, jota sen ei ollut tarkoitus testata.

TCP on tavuvirta, joten yksi `read()`-kutsu voi vastaanottaa vain osan viestistä.
Siksi palvelin lukee dataa silmukassa ja lähettää jokaisen vastaanotetun osan
takaisin `write_all()`-metodilla.

Tässä esimerkissä käytetään TCP-pistokkeiden ominaisuutta, jota emme ole vielä
käsitelleet: yhteys voidaan sulkea osittain eli sen toinen suunta voidaan sulkea
`shutdown()`-metodilla samalla, kun toinen suunta jää käyttöön. Ominaisuutta ei
käytetä kovin usein, mutta tässä testissä siitä on hyötyä. Kun asiakas sulkee
lähetyssuuntansa, palvelin lukee tavuvirran loppumerkin (EOF), lopettaa
suorituksensa ja vapauttaa pistokkeensa. Asiakkaan
[`read_to_end()`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read_to_end)
kerää vastauksen, kunnes palvelimen pistoke sulkeutuu. Palvelintehtävän kahvan
odottaminen tarkistaa myös, että tehtävä päättyi ilman paniikkia. Lopuksi
assert-väittämä tarkistaa, että takaisin lähetetyt tavut vastaavat alkuperäistä
viestiä. Aikaraja saa testin epäonnistumaan, jos viestinvaihto lakkaa etenemästä.

<div class="assignment-frame" markdown="1">

## Tehtävä #6

**Osa 1:** Muokkaa palvelimesi käyttämään **Tokiota** ja async/await-mallia. Jos
olet jo edennyt projektin toteutuksessa etkä halua kirjoittaa koodiasi uudelleen
tai käyttää Tokiota projektissasi, voit myös toteuttaa tätä tehtävää
varten erillisen Tokio-pohjaisen palvelinohjelman, joka käsittelee
perusprotokollan viestit. Kun toteutuksesi toimii, **pohdi raportissa**, mikä
eri lähestymistavoista sopii parhaiten projektisi palvelimen toteutukseen ja
miksi.

**Osa 2:** Laajenna edellisen tehtävän testiasiakasta niin, että se avaa
valittuun palvelimeen vähintään viisi samanaikaista yhteyttä toteuttamiesi
käyttötapausten testaamiseksi. Toteuta tämä luomalla jokaiselle yhteydelle oma
säie. Mittaa ja tulosta myös kunkin säikeen suoritukseen kuluva aika alusta
loppuun esimerkiksi [edellisessä moduulissa](../testing/#erilaisia-metriikoita)
esiteltyjen ajanmittausfunktioiden avulla. Testaa päivitettyä testiasiakasta
kahta [kurssipalvelimella](https://pronets1.dice.aalto.fi/) lueteltua palvelinta
vastaan. Raportoi havaintosi.

Sisällytä raporttiin myös seuraavat tiedot:

- Kuinka paljon aikaa käytit tähän tehtävään?
- Mikä tehtävässä oli helppoa tai vaikeaa?
- Mitä työkaluja tai muita tietolähteitä käytit? Jos käytit tekoälyavustajia,
  kerro erityisesti, miten käytit niitä ja oliko niistä hyötyä.

</div>
