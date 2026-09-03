---
title: Rustin perusteet ja asiakassovelluksen pistokkeet
lang: fi-FI
translation_key: rust-basics
---

Tämän materiaalin esimerkit ja koodi on kirjoitettu Rust-ohjelmointikielellä. Rust on
suhteellisen uusi kieli, joka on kasvattanut suosiotaan esimerkiksi hajautettuja
verkko-ohjelmistoja tekevien kehittäjien keskuudessa. C:n ja C++:n tavoin Rust
käännetään konekielelle, joten Rust-ohjelmien voi olettaa olevan suunnilleen
yhtä tehokkaita. Rustin omistusmalli pyrkii kuitenkin parantamaan
muistiturvallisuutta, mikä on tietoturvan kannalta tärkeää. Kieleen kuuluu myös
nykyaikainen pakettienhallinta ja testaustuki.

<div class="objectives-frame" markdown="1">

**Moduulin tavoitteet:**

- Saat **käsityksen Rustin perusteista** ja harjoittelet kielelle
  ominaisia mekanismeja, esimerkiksi liittyen muistin käsittelyyn.

- **Opit toteuttamaan yksinkertaisen TCP-asiakkaan**, joka muodostaa yhteyden
  palvelimeen sekä lähettää ja vastaanottaa tietoa.

- **Ymmärrät binäärisen tiedon siirtämisen tietokoneen muistin ja
  protokollaviestien välillä.** Erityisesti on tärkeää ymmärtää, että eri
  tietokonearkkitehtuurit voivat esittää binääriluvut eri tavoin.

- **Opit tekemään HTTP-pyynnön ja vastaanottamaan vastauksen** Rustilla.

</div>

## Rustin perusteet

[Rust-kirja](https://doc.rust-lang.org/stable/book/) on kattava Rustin
oppimateriaali ja yleisesitys, johon kannattaa tutustua. Tämä materiaali ei
käsittele kaikkia Rustin erityispiirteitä yksityiskohtaisesti, vaan viittaa
kulloista asiaa käsitteleviin Rust-kirjan lukuihin. Keskitymme ominaisuuksiin, joiden
avulla pääset alkuun verkko-ohjelmoinnissa.

### Rustin ja muiden työkalujen asentaminen

Rust-kirjan [luku 1.1](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)
sisältää asennusohjeet Linuxille, macOS:lle ja Windowsille. Rust käyttää
**rustup**-asennustyökalua, joka asentaa pakettienhallintaan, testaamiseen ja
koodin muotoiluun tarvittavat työkalut.

[VS Code](https://code.visualstudio.com/) on suosittu ohjelmointityökalu, jota
kannattaa käyttää. Lisäksi kannattaa asentaa esimerkiksi **rust-analyzer** ja
**Rust Syntax** - laajennukset VS Codeen. Myöhemmin tarvitset myös **Gitiä** ja
**Dockeria**. Kurssin palautukset työnnetään kurssia varten luotuun
Git-repositorioon, ja palvelinohjelmat suoritetaan Dockerilla yhteisellä
palvelinkoneella.

Kannattaa kokeilla asennuksen jälkeen Rust-kirjan kohdan 1.3 [Hello,
Cargo!](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html)
-esimerkkiä. Cargo on Rustin käännös- ja pakettienhallintatyökalu, jota
käytetään kurssilla paljon. Käytännön tuntumaa saat myös luvun 2
[arvauspelistä](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html).

### Muuttujat, tietotyypit ja funktiot

Kuten useimmat ohjelmointikielet, Rust järjestää ohjelmakoodin funktioihin ja
tallentaa arvot muuttujiin. Suoritus alkaa **main**-funktiosta. Rustissa
muuttuja voi olla vakiomuotoinen, eli sen arvoa ei voi muuttaa asettamisen
jälkeen, tai muutettava jolloin arvon päivittäminen on mahdollista. Arvon
muuttaminen sallitaan avainsanalla **mut**. Rust-kirjan [kohta
3.1](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)
sisältää tästä esimerkkejä.

C:n ja C++:n tavoin Rust liittää jokaiseen arvoon ja muuttujaan kiinteän
**tietotyypin**. Erikokoisille etumerkillisille ja etumerkittömille
kokonaisluvuille sekä liukuluvuille on omat tyyppinsä. Lisäksi Rustissa on
totuusarvo- ja merkkityypit. Katso Rust-kirjan [luku
3.2](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html).
Funktioiden argumentit ja paluuarvot esitellään [luvussa
3.3](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html) ja
ohjausrakenteet [luvussa
3.5](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html).

Seuraava esimerkki määrittelee muuttumattoman merkkijonomuuttujan ja muuttuvan
kokonaislukumuuttujan. Kokonaislukua muutetaan ennen arvojen välittämistä
funktiolle:

```rust
fn make_score_message(name: &str, points: i32) -> String {
    let doubled_points = points * 2;
    format!("{name} has {doubled_points} points")
}

fn main() {
    let player_name = "Ada";
    let mut score: i32 = 10;

    score += 5;

    let message = make_score_message(player_name, score);
    println!("{message}");
}
```

`player_name` on oletusarvoisesti muuttumaton. `score` määritellään ilmauksella
`let mut`, joten sen arvoa voidaan muuttaa. Sen `i32`-tyyppi on kirjoitettu
auki, kun taas Rust päättelee `player_name`-muuttujan tyypin arvosta.
Rust-ohjelmoinnissa on yleistä, että tyyppiä ei erikseen mainita, jos se on
pääteltävissä alustetusta arvosta. Esimerkiksi VS Code ja sen Rust-laajennukset
osaavat kuitenin vinkata ohjelmoijalle minkä tyyppisestä arvosta on kyse.
`make_score_message` kaksinkertaistaa pistemäärän ja palauttaa uuden
`String`-arvon. Ohjelma tulostaa `Ada has 30 points`. Funktion viimeinen rivi ei
pääty puolipisteeseen, koska lausekkeen arvo palautetaan funktiosta. Tämä on
yleinen käytäntö Rustissa, vaikka myös _return_ avainsana on käytettävissä.

Voit kopioida ohjelman tiedostoon `main.rs`, kääntää sen käyttäen `rustc` -
kääntäjää:

    rustc main.rs

Tämä synnyttää "_main_" - nimisen binääritiedoston, joka voidaan nyt suorittaa:

    ./main

Rustissa käytetään yleisesti kahta merkkijonotyyppiä. `String` omistaa
dynaamisesti varatussa puskurissa olevan tekstinsä, ja muuttuvaa `String`-arvoa
voidaan kasvattaa ja muokata. `str` kuvaa merkkijonoa, jota käytetään yleensä
lainatun `&str`-viitteen (eli "merkkijonosiivun", englanniksi "string slice")
kautta. Literaalin `"Ada"` tyyppi on `&str`. Rustin merkkijonot ovat
UTF-8-koodattuja.

### Omistajuus ja viitteet

**Omistajuus** on Rustissa keskeinen käsite. Jokaisella arvolla on omistaja, ja
omistajia voi olla kerrallaan vain yksi. Kääntäjä valvoo omistussääntöjä eikä
käännä niitä rikkovaa ohjelmaa. Tämän johdosta aloittelevalla Rust-ohjelmoijalla
voi olla joskus vaikeuksia saada ohjelma kääntymään, mutta vastineeksi vältetään
monet C-ohjelmien vaikeasti debugattavat ajonaikaiset muistiongelmat.

Myös **näkyvyysalue** on tärkeä käsite, sillä se vaikuttaa muistin varaamiseen
ja vapauttamiseen. Muistia ei vapauteta käsin, vaan se vapautuu automaattisesti,
kun arvon omistaja poistuu näkyvyysalueelta. Katso lisää Rust-kirjan [luvusta
4.1](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html).

Erityisesti funktioiden yhteydessä puhutaan usein **lainaamisesta**, eli arvo
välittämisestä funktiolle viitteenä. Omistajuus säilyy kutsujalla, ja funktio
vain **lainaa** muuttujaa. Viite voi olla muuttumaton tai muuttuva. Rust sallii
muuttujalle näkyvyysalueella vain yhden muuttuvan viitteen kerrallaan, jotta
tiedot pysyvät eheinä. Tämä saattaa aiheuttaa aloittelevalle ohjelmoijalle
joskus päänsärkyä, esimerkiksi toistolauseiden ja silmukoiden yhteydessä jolloin
jotain funktiota saatetaan kutsua toistuvasti. Aihetta käsitellään Rust-kirjan
[luvussa
4.2](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html).

Alla oleva funktio `add_greeting` lainaa `String` - muotoisen merkkijono
muuttuvana viitteenä ja muuttaa sen sisältöä.

```rust
fn add_greeting(name: &mut String) {
    name.insert_str(0, "Hello, ");
    name.push('!');
}

fn main() {
    let mut name = String::from("Ada");

    add_greeting(&mut name);
    println!("{name}"); // Tulostaa: Hello, Ada!
}
```

Kutsuja määrittelee `name`-muuttujan muuttuvaksi ja muodostaa viitteen
ilmauksella `&mut name`. Parametrityyppi `&mut String` sallii lainatun arvon
muuttamisen. Omistajuus säilyy `main`-funktiolla, joten arvoa voi käyttää myös
kutsun jälkeen.

### Rakenteet ja metodit

Monien muiden ohjelmointikielien tapaan, **Rakenteella (struct)** kootaan
toisiinsa liittyvät tiedot yhteen. Rust-kirjan [luku
5.1](https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html)
esittelee rakenteiden määrittelyn ja käytön.

Rakennetta käsittelevät funktiot voidaan määritellä metodeina rakenteen
nimiavaruudessa samaan tapaan kuin luokkia käytetään olio-ohjelmoinnissa.
Metodit määritellään erillisessä `impl`-lohkossa, joka tyypillisesti seuraa
`struct` - määrittelyä, joka vain määrittelee muuttujat ja tietotyypit
rakenteeseen liittyen. Metodit käyttävät yleensä `self`-viitettä, joka viittaa
käsittelyn alla olevaan rakenteen instanssiin. Jos mentodi muokkaa rakenteen
tilaa joillain tavoin, self-viitteen on oltava muuttuva: `&mut self`.
Omistajuuteen liittyvät säännöt pätevät myös _self_ muuttujaan. Rust-kirjan
[luku 5.3](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html)
kertoo lisää.

**Luettelotyyppi (enum)** voi saada yhden useista vaihtoehdoista. Seuraava
esimerkki käsittelee sekä IPv4- että IPv6-osoitteita, sekä luettelon käyttöä
osana rakennetta. Esimerkki on lainattu ja muokattu [Rust-kirjan luvusta
6.1](https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html), jossa
kerrotaan asiasta kattavammin.

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn print_address(&self) {
        println!("{}", self.address);
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    home.print_address();
    loopback.print_address();
}
```

Rustin standardikirjastossa on kaksi erityisen hyödyllistä luettelotyyppiä,
**Option** ja **Result**.

`Option` on joko `None` tai `Some(T)`, jossa T viittaa mihin tahansa
tietotyyppiin. Näin sillä voidaan kuvata arvoa, jota ei aina ole määritelty.
Perinteisesti C-kielessä on käytetty esimerkiksi _null_ - arvoa tällaiseen
tarkoitukseen. Rust-kirjassa keskustellaan siitä, miksi tämä on ongelmallista ja
johtanut vuosien saatossa erilaisiin muistiongelmiin huolimattomasti käytettynä.
Rustissa ei siis ole nollaosoittimia, vaan näissä tilanteissa käytetään
_Option_-tyyppiä.

`Result` on joko `Ok(T)` tai `Err(E)`: onnistunut operaatio voi palauttaa arvon
`T`, ja epäonnistunut virheen `E`, missä _E_ tyypillisesti kertoo lisätietoa
virheestä, esimerkiksi virhekoodin tai merkkijonon muodossa. Tämä on erityisen
hyödyllistä verkko-ohjelmoinnissa, koska ulkoiseen verkkoon kohdistuvat
operaatiot voivat epäonnistua monesta ohjelmoijasta riippumattomasta syystä.
C-kielessä virhetilanteista kerrotaan usein erityisellä lukuarvolla, esimerkiksi
monissa Posix-API:n funktioissa arvo -1 on varattu tähän. Omasta mielestäni
erillinen _Result_ - tyyppi on hienompi tapa hoitaa asia.

Seuraava esimerkki käyttää näitä kahta edellämainittua luettelotyyppiä:

```rust
fn find_address(host: &str) -> Option<String> {
    if host == "localhost" {
        Some(String::from("127.0.0.1"))
    } else {
        None
    }
}

fn parse_port(text: &str) -> Result<u16, String> {
    match text.parse::<u16>() {
        Ok(port) => Ok(port),
        Err(_) => Err(format!("'{text}' is not a valid port number")),
    }
}

fn main() {
    match find_address("example.com") {
        Some(address) => println!("Address: {address}"),
        None => println!("Address was not found"),
    }
    match parse_port("not-a-number") {
        Ok(port) => println!("Port: {port}"),
        Err(error) => println!("Could not parse port: {error}"),
    }
}
```

Esimerkki näyttää myös `match`-lausekkeen, jolla eri vaihtoehdot käsitellään.
Kääntäjä vaatii molempien mahdollisten vaihtoehtojen huomioimisen. Esimerkissä
ensimmäinen `find_address` - kutsu palauttaa _None_ - arvon, ja jälkimmäinen
`parse_port` - kutsu virhearvon. Mikäli kutsuissa olisi käytetty esimerkiksi
arvoja "localhost" ja "8080", oltaisiin päädytty _Some_ ja _Ok_ haaroihin.

### Kokoelmat

Kokoelmat ovat dynaamisia tietotyyppejä, joihin tallennetaan vaihteleva määrä
arvoja. Kokoelman tarvitsema muisti varataan dynaamisesti tietokoneen keosta
("heap"), ja muisti vapautetaan automaattisesti kun kokoelma poistuu
näkyvyysalueeltaan.

**Vektori (Vec)** on tietyn tyyppisten arvojen järjestetty lista. Alkioihin
voidaan viitataan indeksillä, ja lisäksi vektorin käsittelyyn on tarjolla useita
funktioita, kuten `push()`, joka lisää uuden arvon vektorin loppuun tai `pop()`,
joka poistaa viimeisen arvon ja palauttaa sen. Rust-kirjan [luku
8.1](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html) ja [Vec-tyypin
dokumentaatio](https://doc.rust-lang.org/std/vec/struct.Vec.html) kertovat
näistä enemmän.

Seuraava esimerkki luo vektorin joka sisältää porttinumeroita ja muokkaa sen
sisältöä:

```rust
fn main() {
    let mut ports = vec![80, 443];
    ports.push(8080);
    println!("First port: {}", ports[0]);

    for port in &ports {
        println!("Port: {port}");
    }
    match ports.pop() {
        Some(port) => println!("Removed port: {port}"),
        None => println!("The vector was empty"),
    }
}
```

`vec!` on makro, jota käytetään vektorin alustamiseen. Tässä tapauksessa Rust
päättelee muuttujan tyypiksi `Vec<i32>`. Silmukka lainaa _ports_ - vektorin,
joten se on käytettävissä myös myöhemmin funktiossa. `pop()` - funktio palauttaa
`Option` tyyppisen arvon, joka on `Some(port)` jos vektorissa oli arvo joka
poistettiin, tai `None` jos vektori oli tyhjä.

Vektoria ei pidä sekoittaa **taulukkoon**, jolla on ennalta määriteltu pituus,
joka on osa sen tyyppiä:

```rust
fn main() {
    let mut numbers = [5, 6, 7, 8]; // Tyyppi päätellään: [i32; 4]
    numbers[0] = 4;
    for number in numbers {
        println!("number: {number}");
    }
}
```

**HashMap** liittää valitun tyyppiset avaimet tietyn tyyppisiin arvoihin ja
mahdollistaa tehokkaan haun avaimella. Arvot asetetaan muistiin
hajautusfunktiolla, ja siksi alkioiden järjestys ei ole määritelty. Uusia
avain/arvo - pareja voidaan lisätä `insert()`-funktiolla. Kukin avain voi olla
hajatustaulussa vain kerran: jos tietty lisättävä avain oli jo taulussa, se
korvautuu uudella. Lisää asiasta Rust-kirjan [luvussa
8.3](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html).

Seuraava esimerkki käyttää hajautustaulua pitämään kirjaa nimetyistä
palveluista, ja niitä vastaavista porttinumeroista:

```rust
use std::collections::HashMap;

fn main() {
    let mut service_ports: HashMap<&str, u16> = HashMap::new();
    service_ports.insert("http", 80);
    service_ports.insert("https", 443);

    match service_ports.get("https") {
        Some(port) => println!("HTTPS uses port {port}"),
        None => println!("HTTPS was not found"),
    }
    for (service, port) in &service_ports {
        println!("{service}: {port}");
    }
}
```

Tyyppi `HashMap<&str, u16>` käyttä merkkijonosiivuja avaimina, ja etumerkittömiä
16-bittisiä lukuarvoja arvoina. `get()` - metodi palauttaa `Option` - tyypin,
koska on mahdollista, että haettua avainarvoa ei löydy. `HashMap`-tyypin
iterointi tapahtuu avain-arvo parien avulla, joiden järjestystä ei ole
määritelty.

### Rust-projektin perustaminen

Laajemmissa projekteissa käytetään **cargoa**-työkalua moniin erilaisiin
tehtäviin. Sen avulla voidaan kääntää ja suorittaa projekti, käsitellä
kirjastoriippuvuuksia, siistiä koodityyliä, ajaa testejä, jne.

Uusi projekti luodaan komennolla:

    cargo new my_project

Komento luo projektin ominaisuudet ja kirjastoriippuvuudet määrittelevän
`Cargo.toml`-tiedoston, `src`-hakemiston joka sisältää lähdekoodit, sekä sinne
alustavan `main.rs`-tiedoston. Lisäksi luodaan paikallinen Git-repositorion ja
`.gitignore`-tiedosto. `cargo build` kääntää projektin ja luo binääritiedoston
(`target`-hakemistoon projektipuussa) ja `cargo run` suorittaa sen sekä kääntää
tarvittaessa ne osat projektista joita on päivitetty.

Cargo sisältää myös muita toimintoja, joista osaa käsittelemme hieman myöhemmin
[Cargo-kirja](https://doc.rust-lang.org/cargo/index.html) sisältää kattavan
kuvauksen kaikista ominaisuuksista ominaisuudet.

Kun ohjelma kasvaa, se kannattaa jakaa erillisiin moduleihin ja
lähdetiedostoihin, joilla on hyvin määritellyt julkiset rajapinnat jotka
piilottavat toteutusten yksityiskohdat, kuten hyviin ohjelmointikäytäntöihin
kuuluu kielestä riippumatta. Rust-kirjan [luku
7](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
kertoo pakkauksista (package), koreista (crate), ja ohjelman jakamisesta
hierarkisiin moduleihin.

## Virtapistokkeiden perusteet asiakassovelluksissa

Sovellus lähettää ja vastaanottaa tietoa käyttöjärjestelmän ytimelle
**pistokkeen (socket)** kautta. Käyttöjärjestelmä kapseloi tiedon
protokollapaketteihin ja välittää ne verkkoon. Pistoke käsittelee tavallisesti
yhtä paikallisen sovelluksen ja verkossa olevan solmun välistä
viestintäistuntoa, useimmiten TCP-yhteyttä käyttäen. Kurssilla keskitytään TCP:n
luotettaviin **virtapistokkeisiin (stream socket)** ja UDP:tä käyttäviin
epäluotettaviin, sanomamuotoisiin **datagrammipistokkeisiin (datagram socket)**.
Näiden pistokkeiden avulla voisi käyttää myös eräitä muita protokollia.

![Sovelluksen ja käyttöjärjestelmän vuorovaikutus pistokkeiden avulla](/images/basics-socket.svg){: width="90%" .center-img }

Pistokerajapinta määriteltiin alun perin POSIX-rajapinnassa C-kielellä. Rustin
standardikirjasto paketoi toiminnot Rust-ohjelmoijalle sopiviksi (ja hieman
helppokäyttöisemmiksi) funktioiksi.

Aloitamme tässä osiossa virtapistokkeista, ja käsittelemme datagrammipistokkeet
hieman myöhemmin kurssilla. **Virtapistoke** tarjoaa luotettavan tavuvirran
kahden päätepisteen välille. Toinen päätepisteistä on **palvelin**, joka
kuuntelee passiivisesti yhteydenottopyyntöjä sovitussa IP-osoitteessa ja
portissa. Kun uusi yhteydenotto tulee, palvelin luo aktiivisen pistokkeen, jota
käytetääm varsinaiseen kommunikointiin tietyn asiakkaan kanssa. Yhteyden toinen
pää on **asiakas** joka tekee aloittaa yhteyden osoitteeseen, jossa se olettaa
palvelimen odottavan yhteyspyyntöjä. Aloitamme käsittelyn asiakaspäästä, ja
siirrymme palvelinpistokkeisiin kurssin seuraavassa moduulissa.

### Yhteyden muodostaminen

Rustissa TCP-asiakkaan virtapistoke muodostetaan yleensä `connect()`-funktiolla:

```rust
TcpStream::connect("some.address.fi:5000");
```

Merkkijonoparametrin käyttö on useimmissa tapauksessa helpointa, mutta Rust
tukee myös muita muotoja osoitteen välittämiseen. Parametri kertoo koneen
osoitteen sekä TCP-portin johon yhteys avataan. Osoite voi olla DNS-nimi, missä
tapauksessa ensin tehdään DNS-kysely, ennenkuin voidaan avata varsinainen
TCP-yhteys. Toisaalta voidaan määritellä myös suoraan IP-osoite, jolloin kyselyä
ei tarvitse tehdä. [TcpStreamin
dokumentaatio](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html)
kertoo miten eri tavoin tätä tyyppiä voidaan käyttää, ja antaa joitain
esimerkkejä.

`connect()` palauttaa `Result`-arvon, koska yhteyden avaus voi epäonnistua
erinäisitä syistä johtuen, esimerkiksi jos nimikysely ei onnistu, jos osoite on
jotenkin väärin määritelty, tai jos toiseen päähän ei saada muodostettua
yhteyttä. Jos yhteys onnistuu, funktio palauttaa pistokeinstanssin, jota
käytetään kommunikointioperaatioihin jatkossa.

Kuten useimpien tietoliikennefunktioiden kanssa, `connect()` - funktion suoritus
voi kestää pitkään, koska siihen liittyy kommunikointia verkon yli. Itse asiassa
aika paljon ehtii tapahtua:

- Jos parametrina oli DNS-nimi, asiakasjärjestelmä aloittaa nimikyselyn. Jos
  nimi ei löydy paikallisesta välimuistista, tämä saattaa vaatia useita
  DNS-viestejä eri palvelimille, kuten aiemmin kuvattiin

- Kun IP-osoite on saatu selville, TCP aloittaa kolmivaiheisen kättelyn. Koska
  kommunikointi tapahtuu Internetin yli, siinä on viivettä ja paketteja voi
  kadota. Jos paketteja katoaa, käyttöjärjestelmän TCP-toteutus joutuu tekemään
  uudelleenlähetyksiä ajastimen avulla. Näitä tehdään tarvittaessa useita,
  kunnes jossain vaiheessa luovutetaan. Ajastimen pituus kaksinkertaistuu
  jokaisella yrityksellä, kunnes jossain vaiheessa luovutetaan.

Huonolla tuurilla `connect()`-kutsu saattaa pysäyttää ohjelman suorituksen
kymmeniksi sekunneiksi, jos kommunikoinnissa on ongelmia, kunnes yritys joko
onnistuu tai epäonnistuu. Tämä on asia joka pitää ottaa huomioon asiakasohjelmaa
suoritettaessa, jossa siinä on interaktiivisia tai aikakriittisiä toimintoja.

Alla oleva kaavio näyttää DNS-kyselyn ja TCP-kättelyn etenemisen. DNS-kysely
tehdään ennalta konfiguroidulle DNS-palvelimelle. Kun (jos) yritys onnistuu,
TCP-kättely aloitetaan, ja jos se saadaan onnistuneesti päätökseen, päästään
aloittamaan varsinainen kommunikointi. Asiakasjärjestelmä valitsee vapaan
paikallisen TCP-portin automaattisesti (esimerkissä 51782). Palvelinpuolen
funktiot näkyvät kuvassa myös. Niihin palataan seuraavassa moduulissa.

![TCP-yhteyden muodostaminen](/images/basics-tcpconnect.svg){: width="90%" .center-img }

### Tiedon kirjoittaminen pistokkeeseen

Virtapistoke siirtää tavuvirtaa jolle ei oleteta mitään rakennetta. Tämä on
yleinen viestintämalli monessa perinteisessä verkkosovelluksessa, kuten
tiedoston siirrossa (jollaista web-liikenne pitkälti on). Virtapistoke ei
huomioi viestirajoja eikä välitä siitä, monellako `write()`-kutsulla tavut on
kirjoitettu. Pistokkeen takana oleva TCP-toteutus takaa, että perille pääsevät
tavut ovat **alkuperäisessä järjestyksessä** eivätkä **vahingoittuneita**. Tätä
varten TCP käyttää tarkistussummia, uudelleenlähetyksiä ja puskurointia yhteyden
molemmissa päissä. Sovellukselle mahdolliset ongelmat viestinnässä näkyvät
vaihtelevan pituisina viiveinä.

Käyttöjärjestelmä ylläpitää pistokkeen **lähetys- ja vastaanottopuskureita**.
`write()`-funktio **kopioi tiedon lähetyspuskuriin**, jonka jälkeen funktiokutsu
valmistuu, ja ohjelma voi jatkaa suoritustaan. Toisin sanoen, on tyypillistä,
että kun write()-kutsu valmistuu, **sen lähettämää tietoa ei ole vielä välitetty
verkkoon**, vaan se odottaa edelleen käyttöjärjestelmän puskurissa.

Käyttöjärjestelmän on tehtävä useita asioita lähettääkseen puskurissa oleva
tieto:

- Tieto on pilkottava TCP-segmentteihin, eli IP-paketteihin jotka tullaan
  lähettämään verkkoon.

- Ennenkuin paketit voidaan lähettää, TCP-lähettäjän on varmistuttava, että
  vastaanottajan puskurissa on riittävästi tilaa tiedon vastaanottamiseen. Tätä
  varten vastaanottajan on kerrottava TCP-kuittauksissa **vastaanottajan
  ikkunansa**, toisin sanoen kuinka paljon sillä on tilaa omassa puskurissaan.
  Tätä kutsutaan **vuonvalvonnaksi**.

- TCP-lähettäjä sovetaa lähettämiseen **ruuhkanhallintaa**, eli se pyrkii
  sopeuttamaan lähetysnopeutensa arvioimansa verkon kapasiteetin mukaisesti.
  Perinteisessä ruuhkanvalvonnassa TCP käyttää **ruuhkaikkunaa**, joka kertoo
  kuinka paljon kuittaamatonta dataa saa olla kerrallaan siirrossa verkossa. Jos
  ruuhkaikkuna täyttyy, TCP-lähettäjän on viivästettävä lähettämistään jonkun
  aikaa.

Lähetetty tieto säilyy lähettäjän puskurissa, kunnes TCP-kuittaus vahvistaa sen
vastaanotetuksi. Tämän jälkeen lähettäjä voi poistaa tiedon omasta puskuristaan.
Lähettäjä **ei kuitenkaan voi tämän perusteella vielä tietää, onko vastaanottava
sovellus lukenut tiedon** omasta puskuristaan.

Jos lähetyspuskuri on täysi, `write()`-kutsun suoritus pysähtyy määrämättömäksi
ajaksi, kunnes puskuriin tulee tarpeellinen määrä tilaa. Kutsu ei myöskään
välttämättä kirjoita kaikkea tietoa kerralla: jos puskurissa oli tilaa vain
osalle tiedosta, se kopioidaan puskuriin, ja kutsu antaa paluuarvonaan
tavumäärän joka kopioitiin. Jos tämä on vähemmän kuin mitä alunperin pyydettiin
kirjoittamaan, sovelluslogiikan on pystyttävä jatkamaan kirjoittamista oikeasta
kohtaa seuraavilla `write()`-kutsulla.

Yksinkertaisempi funktio `write_all()` tekee logiikasta helpomman pysähtymällä
kunnes kaikki pyydettu tieto on kopioitu.

### Tiedon lukeminen pistokkeesta

`read()`-funktio toimii päinvastoin: se kopioi tietoa pistokkeen
vastaanottopuskurista sovelluksen antamaan puskuriin. Jos luettavaa tietoa ei
ole, kutsu voi pysähtyä määräämättömän pituiseksi ajaksi, kunnes jotain
luettavaa on. Jos tietoa on vähemmän kuin sovelluksen puskurissa tilaa, kutsu
palaa ja ilmoittaa kopioitujen tavujen määrän.

### Esimerkki

Seuraava lyhyt esimerkki käyttää edellä kuvattuja toimintoja. Kurssimateriaalin
git-repositoriossa,
[examples-hakemistossa](https://github.com/PasiSa/pronets/tree/main/examples) on
myös kokonainen esimerkki
**[simple-client](https://github.com/PasiSa/pronets/tree/main/examples/simple-client/src/main.rs)**.
Esimerkki sisältää kaikki tarvittavat Rust-projektitiedostot sen kääntämiseen.
Voit siis testata sitä kloonaamalla repositorion omaan koneeseesi ja kokeilla
ohjelmaa normaaleilla Rust-työkaluilla.

```rust
// io tarvitaan io::Result-paluutyyppiin. Read ja Write tuovat
// read_to_string()- ja write_all()-metodit näkyvyysalueelle.
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let address = "localhost:5000";
    let mut stream = TcpStream::connect(address)?;
    println!("Connected to {address}");

    let message = "Hello there!";
    stream.write_all(message.as_bytes())?;

    // Varaa 160 tavun puskuri ja lue siihen pistokkeesta.
    let mut buf: [u8; 160] = [0; 160];
    let n = stream.read(&mut buf)?;
    println!("Read {} bytes: {}", n, String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
```

`std::io`-moduulista tuodut `Read` ja `Write` ovat **piirteitä (trait)**. Piirre
määrittelee käyttäytymisen, jonka eri tyypit voivat toteuttaa. Se siis
muistuttaa Javan _rajapintaa (interface)_ tai C++:n _puhdasta abstraktia
luokkaa (pure abstact class)_. `TcpStream` toteuttaa molemmat piirteet. `Read`
mahdollistaa tavujen vastaanottamisen, esimerkiksi
`read_to_string()`-funktiolla, ja `Write` lähettämisen, esimerkiksi
`write_all()`-funktion.

Piirteet tarvitaan yhteinäisen rajapinnan tarjoamiseksi, koska `TcpStream` -
toteuttaa pelkästään TCP-kohtaiset toiminnot. `Read` ja `Write` - piirteet
tarjoavat yhtenäisen rajapinnan kaikkille virtamuotoista tietoa tukeville
tyypeille.

Piirteet on myös tuotava näkyvyysalueelle, jotta kääntäjä löytää niiden metodit.
Tätä varten koodin alussa annetaan `use std::io::{Read, Write};`. Ilman tätä
määrittelyä kääntäjä tietäisi että `stream` on TcpStream - tyyppinen, mutta ei
tuntisi piirteiden määrittelemiä metodeja lukemiseksi ja kirjoittamiseksi.

## Binäärisen tiedon koodaus

Luvut ja merkit tallennetaan muistiin binäärisenä tietona. Tavu eli oktetti on
kahdeksan bittiä ja voi esittää etumerkittömät arvot 0–255 tai etumerkilliset
arvot −128–127.

### Luvut ja tavujärjestys

Suuremmat, 16-, 32- ja 64-bittiset kokonaisluvut koostuvat useista tavuista.
Nämä tavut voidaan tallentaa tietokoneen muistiin usealla tavalla. **Big
endian** -järjestyksessä merkitsevin tavu tallennetaan muistiin ensin ja
**little endian** -järjestyksessä vähiten merkitsevä tavu ensin. Useimmat
nykyiset työpöytä- ja palvelinkoneet, kuten x86 prosessorit tai nykyiset Applen
piirit käyttävät sisäisesti little endian -järjestystä, mutta perinteisesti
tässä on ollut vaihtelua.

Verkkoprotokollissa on useimmite sovittu käytettävän big endian -järjestystä,
jota kutsutaan siksi myös **verkon tavujärjestykseksi**. Paikallisen koneen
järjestystä kutsutaan **isäntäkoneen tavujärjestykseksi**. Riippuen
prosessoriarkkitehtuurista nämä kaksi järjestystä voivat siis olla samat, tai
poiketa toisistaan.

Kun binäärisiä kokonaislukuja lähetetään verkkoon, ne pitää siis muuttaa
verkkotavujärjestykseen ennen pistokkeeseen kirjoittamista. Rustin
kokonaislukutyypit tarjoavat apufunktioita tähän. Esimerkiksi `to_be_bytes()`
muuttaa kokonaisluvun tavutaulukoksi joka on big-endian järjestyksessä.

```rust
use std::io::{self, Write};
use std::net::TcpStream;

fn send_number(stream: &mut TcpStream, value: u32) -> io::Result<()> {
    let bytes = value.to_be_bytes();
    stream.write_all(&bytes)?;
    Ok(())
}
```

Vastaavasti verkosta luetut big endian -tavut muunnetaan kokonaislukuarvoksi
`from_be_bytes()`-funktiolla:

```rust
use std::io::{self, Read};
use std::net::TcpStream;

fn receive_number(stream: &mut TcpStream) -> io::Result<u32> {
    let mut bytes = [0u8; 4];
    stream.read_exact(&mut bytes)?;
    Ok(u32::from_be_bytes(bytes))
}
```

### Tietorakenteiden käsittely

Kun Rust kääntää tavaillisen rakenteen, se voi lisätä kenttien väliin
ylimääräisiä tavuja, jotta prosessori käsittelee kenttiä tehokkaasti. Nämä tavut
kuuluvat rakenteen muistiesitykseen mutta tavallisesti eivät verkkoprotokollan
viestiin. Myös rakenteessa olevat yli 8-bittiset kokonaisluvut on edelleen
muunnettava verkon tavujärjestykseen.

Esimerkkihakemiston esimerkki
**[tcpheader](https://github.com/PasiSa/pronets/tree/main/examples/tcpheader/src/main.rs)**
muuntaa TCP-otsakkeen rakenteesta standardin mukaiseksi tavutaulukoksi ja
koostaa rakenteen saapuvasta tavutaulukosta.

### Teksti ja merkkijonot

Verkkoon lähetettävä teksti on muunnetava tavuiksi, jotta funktiorajapintojen
tyyppimäärittelyitä voidaan noudattaa. Tätä varten protokollamäärittelyn on
kerrottava mitä merkkikoodausta käytetään. Perinteisesti tekstiprotokollat
käyttivät 7-bittistä ASCIIa, joka pystyi esittämään rajallisen määrän
perusmerkkejä. Nykyään käytetään usein UTF-8:aa, joka esittää Unicode-tekstin
8-bittisten tavujen jonona mutta säilyttää tavalliset ASCII-merkit ennallaan.

Rustin merkkijonot ovat UTF-8-koodattuja, se tarjoaa apufunktioita merkkijonojen
muuntamiseksi tavujonoksi ja päinvastoin (koska nämä ovat eri tietotyyppejä).

```rust
use std::io::{self, Write};
use std::net::TcpStream;

fn send_text(stream: &mut TcpStream, text: &str) -> io::Result<()> {
    stream.write_all(text.as_bytes())?;
    Ok(())
}
```

## HTTP:n perusteet

**Hypertext Transfer Protocol (HTTP)** on nyky-Internetin luultavasti yleisin
sovellusprotokollia. Sillä siirretään web-sisältöä TCP-protokollan päällä ja
sitä käytetään myös yleisempänä kommunikointiprotokollana sovellusten välillä,
esimerkiksi REST-rajapintojen viestintäprotokollana. Salaamaton HTTP käyttää
tavallisesti palvelimella porttia 80 (jolloin sisältöjä voi lukea esimerkiksi
Wiresharkin avulla) ja TLS-salattu HTTPS porttia 443 palvelinpäässään. Nykyiset
toteutukset käyttävät käytännössä lähes aina salattua yhteyttä.

HTTP on pyyntö–vastausprotokolla. Asiakas lähettää HTTP-metodin (`GET`, `POST`,
`PUT` jne.), otsakkeita ja mahdollisesti rungon sisältävän pyynnön. Palvelin
vastaa numeerisella tilakoodilla, otsakkeilla ja rungolla.

Tim Berners-Lee ja muut CERNin tutkijat kehittivät HTTP:n vuonna 1991 (katso
[julkaisu](https://dl.acm.org/doi/abs/10.1145/179606.179671)), ja suurimman osan
sen jälkeisestä ajasta on käytetty protokollan tekstimuotoisia versioita 1.0 ja
1.1. 2010-luvun alussa [HTTP/2](https://datatracker.ietf.org/doc/html/rfc7540)
määriteltiin. Se toi merkittäviä muutoksia protokollaan, muun muassa otsakkeiden
binäärikoodauksen, jolloin viestit mahtuivat pienempään tilaan. Myöhemmin tuli
vielä [HTTP/3](https://datatracker.ietf.org/doc/html/rfc9114), joka perustuu
UDP:n päällä toimivaan **QUIC-protokollaan**.

Kurssin tehtävissä keskitytään HTTP/1:een, vaikka siitä ollaan monin paikoin
luopumassa, koska tekstimuotoisten viestien käsittely on kurssilla käytettävien
työkalujen avulla helpompaa.

Yksinkertaisimmillaan HTTP/1.1:n GET-pyyntö pyytää palvelimelta jotain
verkkoresurssia (esim. HTML-websivu). Pyyntöviestissä on aloitusrivi, jonka
jälkeen seuraa vaihteleva määrä otsakekenttiä omilla riveillään. Tyhjä rivi
otsakekenttien jälkeen kertoo että otsakkeen loppuvat ja viestin runko-osa
alkaa. Toisin kuin joissain muissa viestityypeissä, GET-pyyntöviestissä ei ole
runko-osaaa lainkaan.

HTTP/1.1:n rivit päättyvät kahden tavun CRLF-jaksoon, joka kirjoitetaan
Rust-merkkijonossa `\r\n`. Tyhjä rivi päättää otsakeosan, joten ohjelmakoodissa
pyyntö päättyy jaksoon `\r\n\r\n`.

```http
GET /index.html HTTP/1.1
Host: example.com

```

Vastauksviestin alussa on protokollaversio, tilakoodi ja tilateksti. Kuten
pyyntöviestissäkin, vastauksessa ensimmäistä riviä seuraaavat otsakkeet ja
tyhjän rivin jälkeen varsinainen runko. `Content-Length` ilmoittaa rungon
pituuden, jotta vastaukset voidaan erottaa toisistaan. Kuten aiemmin
käsiteltiin, TCP on virtaprotokolla, joten pistokerajapinnan `read` ei
välttämättä palauta koko vastausta sellaisenaan.

```http
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 13

Hello, world!
```

POST-pyyntö lähettää tietoa palvelimelle. Tässä tapauksessa käytetään
JSON-koodausta viestin rungossa nimettyjen tietoalkioiden esittämiseen. JSON on
usein käytössä HTTP-pohjaisissa rajapinnoissa, ja mekin tulemme käyttämään sitä
kommunikoinnissa kurssipalvelimen kanssa.

```http
POST /new-user HTTP/1.1
Host: example.com
Content-Type: application/json
Connection: close

{
    "name": "Alice",
    "age": 30,
    "email": "alice@example.com"
}
```

<div class="assignment-frame" markdown="1">

## Tehtävä #2

Tämä tehtävä koostuu useasta osasta. Ensin toteutetaan TCP-asiakas, joka
muodostaa yhteyden osoitteeseen valitsemallesi web-palvelimelle porttiin 80 ja
tekee GET-pyynnön resurssille `/index.html` (voit käyttää esimerkiksi samaa
osoitetta kuin edellisessä harjoituksessa). Toisin sanoen käytämme salaamatonta
HTTP:tä (vaikka sitä ei todellisissa sovelluksissa nykyään suositella, mutta
emme osaa toteuttaa vielä TLS-salausta). Luo aiemmin tekemääsi Git-repositorioon
hakemisto "**http-client**" ja sijoita asiakkaan lähdekoodi sinne nimellä
`main.rs`.

Kirjoita MyCourses-palautukseen lyhyt raportti, jossa kuvaat etenemisesi
seuraavien askelmien mukaisesti ja vastaat kysymyksiin joita matkan varrella
esitetään.

1. Avaa Wireshark ja kaappaa UDP-porttiin 53 ja TCP-porttiin 80 menevät
   paketit.

2. Kun ohjelma on muodostanut yhteyden palvelimeen, pysäytä se odottamaan
   käyttäjän syötettä ennen varsinaisen HTTP-pyynnön lähettämistä. Voit käyttää
   esimerkiksi `read_line()`-funktiota käsittelemättä annettua syötettä:

```rust
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
```

{:start="3"}

3. Paikanna DNS-pyyntö ja -vastaus. Mihin IP-osoitteeseen olet ottamassa
   yhteyttä? Onko se IPv4 vai IPv6?

4. Tunnista TCP-yhteys Wiresharkissa. Mitä lähdeporttia yhteys käyttää? Mitkä
   TCP-optiot näkyvät ensimmäisessä SYN-paketissa?

Kun olet tarkistanut DNS-kyselyn ja -vastauksen, paina sovelluksessasi Enteriä.
Anna ohjelman lähettää HTTP-pyyntö, vastaanottaa vastaus ja tulostaa se
vakiotulosteeseen.

{:start="5"}

5. Mikä vastauksen tilakoodi on ja mitä se tarkoittaa? Mitä lisätietoja
   otsakkeet kertovat?

Laajenna ohjelmaa toisella HTTP-pyynnöllä. Tee tällä kertaa POST-pyyntö, joka
lähettää palvelimelle tietoja ja saa sen hakemaan annetun Git-repositorion.
Lähetä pyyntö palvelimen `pronets1.dice.aalto.fi` porttiin 80 ja polkuun
`/fetch-git`. Tästä eteenpäin Wiresharkia ei tarvitse analysoida, mutta voit
toki jättää sen seuraamaan liikennettä.

Pyynnön rungon on oltava JSON-muotoinen ja sen `Content-Type`-otsakekentän on
oltava `application/json`, kuten aiemmassa esimerkissä. Rungossa on seuraavat
avaimet:

- **`name`**: projektin nimi. Myöhemmin jaamme projekteille uniikit nimet,
  joita käytetään viesteissä. Tässä vaiheessa voit käyttää mitä tahansa nimeä.
- **`git-repo`**: kurssitehtävissä ja projektissa käyttämäsi Git-repositorion
  SSH-osoite. Löydät sen version.aalto.fi-palvelun sinisen _Code_-painikkeen
  alta.

{:start="6"}

6. Tulosta palvelimen vastaus. Pyynnön käsittely voi kestää hetken, koska
   palvelin yrittää kloonata repositoriosi. Onnistuiko pyyntö vastauksen mukaan?
   Jos ei, kerro se raportissa ja yritä korjata tilanne. Repositoriossa on
   oltava vähintään yksi commit, jotta palvelin voi hakea sen.

Kun koodi on valmis, tee commit ja työnnä se repositorioosi. Kerro
commit-viestissä tehtävän 2 valmistumisesta, esimerkiksi _Assignment 2
completed_.

Vastaa lopuksi seuraaviin kysymyksiin:

- Kuinka paljon aikaa käytit tehtävään?
- Mikä tehtävässä oli helppoa tai vaikeaa?
- Mitä työkaluja käytit? Jos käytit tekoälyavustimia, kerro miten käytit niitä
  ja olivatko ne hyödyllisiä.

</div>
