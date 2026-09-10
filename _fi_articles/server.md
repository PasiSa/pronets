---
title: Palvelinohjelmointi ja Docker-kontit
lang: fi-FI
translation_key: server
---

Asiakasohjelmiin verrattuna palvelinohjelmointiin liittyy erilaisia
ominaisuuksia ja haasteita. Palvelimen täytyy yleensä toimia taustalla pitkään
ilman keskeytyksiä, vastata tehokkaasti useiden asiakkaiden yhteydenottoihin ja
hallita useita asiakasistuntoja rinnakkain. Rinnakkaisuuden hallinta muun
sovelluslogiikan ohessa on yksi tehokkaan palvelintoteutuksen keskeisistä
haasteista. Koska palvelimen täytyy toimia tehokkaasti pitkiä aikoja, jopa
viikkoja, ilman käyttäjän vuorovaikutusta, muistihallinnan turvallisuus on
tärkeää. Huolimaton muistinhallinta on aiheuttanut monia
tietoturvahaavoittuvuuksia, joista osa on pysynyt pitkään huomaamatta.
Muistivuodot puolestaan romahduttavat lopulta palvelimen suorituskyvyn. Tämä on
yksi syy siihen, miksi Rust on viime aikoina kasvattanut suosiotaan
verkko-ohjelmistojen kehittäjien keskuudessa.

Tämän moduulin loppupuolella tutustutaan Dockeriin, joka on yleisesti käytetty
teknologia palvelinten paketoimiseen ja käyttöönottoon skaalautuvalla ja
siirrettävällä tavalla. Käytämme Dockeria opiskelijoiden palvelintoteutusten
ajamiseen kurssin palvelimella, jossa palvelinohjelmia voidaan testata. Tästä
eteenpäin tehtävät laajentavat verkkoprojektia, joka laajenee kurssin
loppuun mennessä kokonaiseksi verkkosovellukseksi.

<div class="objectives-frame" markdown="1">

**Tämän moduulin tavoitteet:**

- **Opit passiivisten palvelinpistokkeiden perusteet** ja niiden käytön.

- **Opit toteuttamaan iteratiivisen yksisäikeisen palvelimen**, joka pystyy
  käsittelemään useita samanaikaisia asiakkaita blokkaamattomien pistokkeiden
  avulla.

- **Ymmärrät konttiteknologian perusteet.**

- **Opit rakentamaan Docker-kuvan**, joka suorittaa Rust-lähdekoodista
  käännettyä palvelinohjelmaa.

</div>

## Passiiviset ja aktiiviset pistokkeet

Kun yhteyspohjainen asiakaspistoke avataan tiedonsiirtoa varten, sitä kutsutaan
**aktiiviseksi pistokkeeksi**. Aktiivista pistoketta voidaan käyttää sekä tiedon
lähettämiseen että vastaanottamiseen, ja se on sidottu sekä paikalliseen että
etäpään IP-osoitteeseen ja kuljetuskerroksen porttiin.

Palvelinsovellus sen sijaan avaa pistokkeen aluksi _passiivisessa_ tilassa.
**Passiivista pistoketta** ei ole vielä liitetty toisen pään pistokkeeseen. Se
on sidottu ainoastaan paikalliseen IP-osoitteeseen ja porttiin, joissa palvelin
kuuntelee saapuvia yhteyspyyntöjä. Asiakkaan täytyy tietää tämä osoite, jotta se
voi muodostaa yhteyden palvelimeen. Passiivista pistoketta ei voida käyttää
tiedon lähettämiseen tai vastaanottamiseen.

Rustissa palvelin valitsee IP-osoitteen ja portin sekä avaa pistokkeen
passiivisessa tilassa `bind`-kutsulla. Nykyaikaisissa järjestelmissä samalla
laitteella on tavallisesti käytössä useita eri verkkorajapintoihin kuuluvia
IP-osoitteita. Esimerkiksi kannettavalla tietokoneella on laitteen sisäiseen
viestintään osoite 127.0.0.1, ja lisäksi siinä voi olla eri
IP-osoitteita käyttävät langaton ja langallinen lähiverkkoliitäntä. IP-osoite
sidotaan usein **mihin tahansa** osoitteeseen eli IPv4:n tapauksessa
osoitteeseen 0.0.0.0. Tällöin saapuvia yhteyksiä vastaanotetaan kaikista
verkkoliitännöistä. Jos sovellus haluaa vastaanottaa yhteyksiä vain tietystä
verkkoliitännästä, pistoke täytyy sitoa kyseisen liitännän osoitteeseen.

```rust
use std::io;
use std::net::TcpListener;

fn main() -> io::Result<()> {
    // Sido TCP-portti 1234 kaikkiin paikallisiin IPv4-verkkoliitäntöihin.
    let listener = TcpListener::bind("0.0.0.0:1234")?;
    println!("Server listening on 0.0.0.0:1234");

    // Odota, kunnes yksi asiakas muodostaa yhteyden.
    let (stream, address) = listener.accept()?;
    println!("Accepted connection from {address}");

    // 'stream' on nyt aktiivinen pistoke vasta yhdistettyyn asiakkaaseen.
    // 'address' sisältää toisen yhteyspään IP-osoitteen ja portin.
    // Sen kautta voidaan lähettää ja vastaanottaa tietoa tavalliseen tapaan.

    Ok(())
}
```

Kun palvelimelle saapuu uusi yhteyspyyntö, sen täytyy hyväksyä pyyntö
`accept`-kutsulla. Kutsu luo uuden **aktiivisen pistokkeen**, jonka kautta
palvelin viestii yhteyden muodostaneen asiakkaan kanssa. Pistokkeelle on
määritetty yhteyden molempien päätepisteiden osoitteet, ja sitä voidaan käyttää
tiedon lähettämiseen ja vastaanottamiseen. Tämän jälkeen pistokkeen toiminta on
symmetristä: molemmat osapuolet voivat lähettää ja vastaanottaa tietoa
haluamallaan tavalla, yleensä jonkin määritellyn protokollan mukaisesti. Ajan
mittaan avoinna voi olla useita aktiivisia pistokkeita uusien asiakkaiden
muodostaessa yhteyksiä. Palvelin tarvitsee siis jonkin tavan samanaikaisten
asiakkaiden oikea-aikaiseen käsittelyyn, sillä oletusarvoisesti luku- ja
kirjoituskutsut voivat estää ohjelman suorituksen määräämättömäksi ajaksi, ellei
rinnakkaisuutta ja ei-blokkaavaa toimintaa ole toteutettu asianmukaisesti.

## Erilaisia palvelintoteutuksia

Palvelin voidaan toteuttaa eri tavoin sovelluksen tarpeiden ja
skaalautuvuusvaatimusten perusteella. Seuraavassa on joitakin yleisiä
toteutustapoja:

- Yksinkertaisin vaihtoehto on yksisäikeinen **iteratiivinen palvelin**, joka
  käsittelee asiakasyhteydet yksi kerrallaan ilman samanaikaisuutta. Tämä voi
  soveltua hyvin pieniin tehtäviin ja yksinkertaisiin testeihin, mutta muutoin
  ei ole kovin hyvä tapa toteuttaa palvelinta.

- Voimme käyttää **blokkaamattomia pistokkeita** ja **I/O-kanavien
  multipleksointia**. Käyttöjärjestelmän tarjoamien mekanismien avulla voidaan
  odottaa tapahtumia useista syötelähteistä, jolloin useita asiakkaita voidaan
  käsitellä samanaikaisesti (mutta ei rinnakkain) yhdessä säikeessä.
  Yksisäikeisen toteutuksen rajoituksena on, että se käyttää vain yhtä
  suoritinydintä eikä siksi välttämättä hyödynnä nykyaikaisen moniydinpalvelimen
  koko kapasiteettia.

- Jokaiselle saapuvalle asiakkaalle voidaan käynnistää **oma säie**. Useiden
  säikeiden avulla voidaan käyttää useita suoritinytimiä, mutta uuden säikeen
  käynnistäminen on käyttöjärjestelmälle verrattain raskas operaatio.

- Voimme käyttää **ennalta luotua säiejoukkoa** ja antaa uudet asiakasyhteydet
  käsiteltäviksi sitä mukaa, kun säikeitä vapautuu edellisistä tehtävistä.

- **Asynkroninen I/O** on async/await-rakenteiseen perustuva ohjelmointimalli,
  jossa operaatiot jaetaan tehtäviksi. Tehtäviä hallitsee ohjelmistokirjaston
  tarjoama, sovellusprosessin sisällä toimiva ajoympäristö. Ajoympäristö voi
  toimia yhdessä säikeessä tai hyödyntää useita säikeitä rinnakkaisuuden ja
  käytettävissä olevien suoritinytimien paremmaksi hyödyntämiseksi.

- Joskus kannattaa käynnistää **erillinen käyttöjärjestelmäprosessi** asiakkaan
  istuntoa varten. Tämä on käyttöjärjestelmälle raskas operaatio, mutta se
  eristää asiakasistunnot hyvin toisistaan. Menetelmä soveltuu esimerkiksi
  SSH:n kaltaisten etäkomentoistuntojen toteuttamiseen.

Nämä toteutustavat eivät sulje toisiaan pois. Tuotantojärjestelmissä yhdistetään
usein eri menetelmiä järjestelmän tarpeiden mukaan. Tässä moduulissa käsitellään
kahta ensimmäistä tapaa, ja monisäikeisyyteen ja asynkroniseen I/O:hon palataan
hieman myöhemmin.

## Yksinkertainen iteratiivinen palvelin

Tutustutaan seuraavaksi GitHub-repositoriomme
**[simple-server](https://github.com/PasiSa/pronets/tree/main/examples/simple-server/src/main.rs)**-esimerkkiin,
joka on luultavasti yksinkertaisin mahdollinen palvelintoteutus. Ohjelma
hyväksyy saapuvat yhteydet yksi kerrallaan, lukee asiakkaan lähettämän tiedon ja
lähettää sen takaisin asiakkaalle. Tämän jälkeen yhteys suljetaan ja palvelin
alkaa odottaa seuraavaa asiakasta. Palvelin saa komentoriviparametrina
IP-osoitteen ja kuljetuskerroksen portin, joihin se sidotaan. Kun IP-osoitteena
käytetään osoitetta "0.0.0.0" (olettaen, että käytössä on IPv4), yhteyksiä
vastaanotetaan kaikista verkkoliitännöistä. Jos kuljetuskerroksen portiksi
annetaan 0, järjestelmä valitsee vapaan portin. Käytännössä tämä on hankalaa,
koska asiakassovellukset eivät silloin tiedä, mihin porttiin niiden pitäisi
muodostaa yhteys.

Käynnistä ensin palvelin esimerkiksi näin:

    cargo run -- 0.0.0.0:2000

Sen jälkeen voit testata palvelinta toisessa terminaali-ikkunassa netcatilla ja
kirjoittaa sille viestin:

    nc 127.0.0.1 2000

Voit myös lähettää viestin toisessa ikkunassa
**[simple-client](https://github.com/PasiSa/pronets/tree/main/examples/simple-client)**-esimerkillä
käyttäen localhost-osoitetta. Voit suorittaa esimerkiksi seuraavan komennon
repositorion simple-client-hakemistossa:

    cargo run -- 127.0.0.1:2000 Hello

Yksinkertainen palvelin aloittaa luomalla passiivisen palvelinpistokkeen ja
sitomalla sen komentoriviparametrissa annettuun osoitteeseen. `server` on
yhteyksiä kuunteleva passiivinen palvelinpistoke.

```rust
let server = TcpListener::bind(&args[1])?;
```

Seuraavaksi palvelin käynnistää silmukan, jonka alussa se odottaa seuraavaa
saapuvaa asiakasta. `accept`-kutsu voi pysäyttää ohjelman suorituksen pitkäksi
aikaa.

```rust
let (mut socket, address) = server.accept()?;
println!("Accepting connection from {}", address.to_string());
```

Kun kutsu päättyy, saadaan yhteyden muodostanutta asiakasta vastaava aktiivinen
`socket`-pistoke sekä asiakkaan osoite, joka tulostetaan terminaali-ikkunaan.

Tämän jälkeen palvelin lukee tietoa aktiivisesta asiakaspistokkeesta olettaen,
että asiakas tietää, että sen odotetaan kirjoittavan jotakin. Jos asiakas ei
kirjoittaisi mitään vaan jäisi odottamaan syötettä muualta, `read`-kutsu
pysäyttäisi ohjelman suorituksen pitkäksi aikaa.

```rust
let mut buf: [u8; 160] = [0; 160];
let readn = socket.read(&mut buf)?;
```

Lopuksi palvelin lähettää lukemansa tiedon takaisin asiakkaalle ja sulkee
pistokkeen, kun paikallisen `socket`-muuttujan elinkaari päättyy silmukan lopussa.

## I/O-kanavien multipleksointi ja blokkaamattomat pistokkeet

Pistoke avataan oletusarvoisesti blokkaavassa tilassa. Tällöin esimerkiksi
`read`- ja `write`-funktiokutsut voivat pysäyttää ohjelman suorituksen, kunnes
lukeminen tai kirjoittaminen on mahdollista. Tämä voi aiheuttaa ongelmia, kun
ohjelman täytyy reagoida muihin syötteisiin, kuten käyttäjän toimintaan tai
useisiin palvelimeen samanaikaisesti yhteydessä oleviin asiakkaisiin.

Pistokkeet voidaan **asettaa blokkaamattomaan tilaan**, jolloin kutsut palaavat
välittömästi. Jos esimerkiksi `read`-funktiolla ei ole luettavaa tietoa ja kutsu
olisi siksi pysäyttänyt ohjelman suorituksen tavallisessa blokkaavassa tilassa,
kutsun blokkaamaton versio palauttaa erityisen **WouldBlock**-"virheen". Kyse ei
oikeastaan ole virheestä, vaan paluuarvo kertoo, ettei mitään ollut luettavissa.
Yksinkertainen toteutustapa olisi rakentaa palvelimeen silmukka, joka lukee
kaikkia pistokkeita tällä tavalla. Se kuitenkin muodostaisi aktiivisen
odotussilmukan, joka kuormittaisi suoritinta tarpeettomasti silloinkin, kun
yhdeltäkään asiakkaalta ei saavu tietoa.

Blokkaamattomien pistokkeiden aiheuttaman tarpeettoman suoritinkuorman
välttämiseksi standardi Posix-rajapinta tarjoaa **poll**-operaation. Sen avulla
voidaan odottaa samanaikaisesti useita I/O-tapahtumia pistokkeista ja muista
I/O-lähteistä. Operaatio blokkaa, kunnes vähintään yksi rekisteröidyistä
tapahtumalähteistä on käytettävissä. Paluuarvosta sovellus saa selville, mitä
pistokkeita ja muita tietolähteitä voidaan käsitellä.

Rustissa [**mio**](https://docs.rs/crate/mio) on kirjasto (Rust-termein
"crate"), joka tarjoaa blokkaamattomien pistokkeiden käsittelyyn kätevän joukon
funktioita.
**[iterative-server](https://github.com/PasiSa/pronets/tree/main/examples/iterative-server/src/main.rs)**-esimerkki
havainnollistaa _Mion_ käyttöä. Esimerkin koodi kannattaa avata erilliseen
ikkunaan tätä osiota luettaessa. Edellisen esimerkin tavoin palvelin lukee
pistokkeesta saapuvan tiedon ja lähettää sen takaisin. Aiemmasta esimerkistä
poiketen palvelin ei sulje pistoketta tiedon lähettämisen jälkeen, vaan jatkaa
asiakkaalle vastaamisen jälkeen uuden tiedon odottamista, kunnes asiakas sulkee
yhteyden. Siksi palvelimen täytyy pystyä käsittelemään useita asiakaspistokkeita
samanaikaisesti.

`main`-funktion ensimmäiset rivit ovat samankaltaiset kuin edellisessä
esimerkissä: sidontaosoite luetaan komentoriviparametereista. Sen jälkeen
alustetaan _Mion_ poll-palvelu ja _Mion_ tapahtumat sisältävä tietorakenne.
Jokaiselle mahdolliselle tapahtumalähteelle annetaan yksilöllinen
tapahtumalähteen tunnistava `Token` (joka käytännössä vain kapseloi
kokonaisluvun). Yksilöllisten tunnisteiden varaamisen ja vapauttamisen
helpottamiseksi erilliseen `tokenmanager.rs`-tiedostoon on toteutettu pieni
`TokenManager` - tyyppi.

Aluksi tapahtumalähteeksi lisätään ainoastaan passiivinen kuuntelupistoke ([rivi
60](https://github.com/PasiSa/pronets/blob/6e0d2f11eb9fdfd06c07322733acb3b109110bd9/examples/iterative-server/src/main.rs#L60)).
Huomaa, että Mion `TcpStream`- ja `TcpListener`-toteutukset eroavat hieman
samannimisten tyyppien standarditoteutuksista. Tämä näkyy ohjelman alussa
olevista `use`-lauseista. Mion toteutukset ovat yhteensopivia Mion kanssa ja
toimivat blokkaamattomasti.

Päätapahtumasilmukan ytimenä on _Mion_ `poll`-funktio ([rivi
71](https://github.com/PasiSa/pronets/blob/6e0d2f11eb9fdfd06c07322733acb3b109110bd9/examples/iterative-server/src/main.rs#L71)),
joka pysäyttää suorituksen, kunnes vähintään yksi tapahtuma on saatavilla.
`poll`-kutsun päättyessä saatavilla voi olla useita tapahtumia, joten ne kaikki
täytyy käsitellä yksitellen. Kuuntelupistokkeen tapahtuma kertoo, että
`accept`-kutsu voidaan tehdä turvallisesti pysäyttämättä ohjelmaa. Pieni
`Client`-rakenne sisältää asiakkaan pistokkeen ja osoitteen. Kaikki aktiiviset
asiakkaat tallennetaan `HashMap`-tietorakenteeseen, jossa avaimena käytetään
Mion `Token`-tunnistetta. Monimutkaisemmassa sovelluksessa `Client`-rakenne
voisi sisältää myös muita sovelluksen tarvitsemia asiakaskohtaisia tietoja. Kun
uusi asiakas hyväksytään, sille varataan uusi tunniste, ja asiakas
rekisteröidään Mioon seurattavaksi tapahtumalähteeksi.

Miolla on erilliset **tapahtumatyypit** tilanteisiin, joissa pistoketta voidaan
**lukea** tai siihen voidaan **kirjoittaa** ohjelman suoritusta blokkaamatta.
Asianmukaisessa toteutuksessa myös `write()`-kutsut pitäisi käsitellä
tapahtumasilmukan kautta, mutta tässä se jätetään yksinkertaisuuden (ja ehkä
laiskuuden) vuoksi tekemättä. Toisaalta kirjoitamme enintään 160 tavua, ja
käyttöjärjestelmän pistokepuskurit ovat yleensä vähintään kymmenien kilotavujen
kokoisia. Siksi on varsin epätodennäköistä, että `write()`-kutsu blokkaisi täyden
pistokepuskurin vuoksi.

Kun asiakasyhteydet on avattu, myös mahdolliset asiakaspistokkeiden tapahtumat
tarkistetaan erillisessä `if`-haarassa. Tässä kannattaa kiinnittää huomiota
`read`-kutsun paluuarvojen käsittelyyn ja `Result`-tyypin käyttöön. Onnistunut
lukeminen palauttaa `Ok`-vastauksen, jonka sisältämä paluuarvo kertoo luettujen
tavujen määrän. Jos arvo on 0, asiakas on sulkenut pistokkeen. Tällöin Mion
tapahtumatunniste täytyy vapauttaa ja asiakas poistaa `HashMap`-tietorakenteesta.
Samalla pistokkeen elinkaari päättyy, joten pistoke suljetaan myös palvelimen
päässä. `Err`-vastaus tarkoittaa, että lukemisen aikana tapahtui virhe. Myös
tällöin asiakaspistoke siivotaan, mutta palvelimen päätapahtumasilmukan toimintaa
ei lopeteta. Aiemmin olemme yleensä käyttäneet `?`-operaattoria, joka välittää
mahdollisen virheen ylöspäin kutsupinossa ja olisi siten johtanut ohjelman
päättymiseen, mutta sitä ei nyt tietenkään kannata käyttää.

`write`-kutsu esittelee toisen tavan tarkistaa virhetilanne silloin, kun tarkka
`Ok`-paluuarvo ei kiinnosta. Sen lisäksi, että kirjoituskutsu käsiteltäisiin
kirjoitusvalmiutta ilmaisevan tapahtuman kautta, parempi toteutus tarkistaisi
myös, kuinka monta tavua todella kirjoitettiin, ja varautuisi siihen, että vain
osa tiedosta saatiin kirjoitettua.

Voit testata ohjelmaa käynnistämällä ensin palvelimen samalla tavalla kuin
aiemmin:

    cargo run -- 0.0.0.0:2000

Avaa sen jälkeen useita pääteikkunoita ja käynnistä jokaisessa netcat-istunto,
jolloin palvelimeen muodostuu useita yhteyksiä:

    nc 127.0.0.1 2000

Kannattaa kokeilla kirjoittaa eri asioita eri asiakaspään terminaaliikkunoissa.
Sulje netcat joissakin ikkunoissa näppäinyhdistelmällä Ctrl-D (yhteyden
katkaisu) tai Ctrl-C (netcatin keskeytys) ja käynnistä netcat sitten uudelleen,
niin näet miten palvelin reagoi.

Yksisäikeisen tapahtumapohjaisen palvelinrakenteen etuna on sen keveys ja
ennakoitava toiminta, kunhan operaatiot eivät blokkaa. Rakenne välttää säikeiden
hallinnan ja synkronoinnin aiheuttaman kuorman. Toisaalta yhden säikeen käyttö
ei välttämättä hyödynnä tehokkaasti kaikkia resursseja tyypillisellä
palvelimellä jolla on monta suoritinta käytettävissä.

## Dockerin perusteet

Dockerilla sovellus voidaan paketoida ajoympäristönsä kanssa niin, että se
toimii yhdenmukaisesti eri laitteilla. Docker-**kontit** käyttävät Linux-
käyttöjärjestelmän mekanismeja, kuten
[nimiavaruuksia](https://en.wikipedia.org/wiki/Linux_namespaces) ja
[cgroup-ryhmiä](https://en.wikipedia.org/wiki/Cgroups), sovellusprosessien
eristämiseen sekä resurssien, tiedostojärjestelmien ja verkkoyhteyksien käytön
hallintaan. Docker-**kuva** sisältää kontin luomiseen ja suorittamiseen
tarvittavan tiedostojärjestelmän ja määritykset. Dockeria käytetään usein
verkkopalvelujen tarjoamiseen esimerkiksi datakeskuksissa, joissa
orkestrointijärjestelmä voi dynaaamisesti hajauttaa, hallita ja skaalata
kontteja useille palvelimille.

Tällä kurssilla Dockeria käytetään
palvelintoteutusten siirtämiseen paikallisilta koneilta Aallon ylläpitämälle
kurssipalvelimelle. Dockerin avulla palvelinsovellusta voidaan ensin testata
paikallisesti ja siirtää se sitten palvelimelle, jossa kaikki pääsevät
kokeilemeen sitä.

Ensin luodaan Docker-kuva, joka sisältää sovelluksen tiedostojärjestelmän ja
tarvittavat metatiedot. `Dockerfile` määrittää ohjeen kuvan rakentamiseen.
Docker-kuvat koostuvat tiedostojärjestelmäkerroksista, jotka yleensä rakentuvat
peruskuvan päälle. Peruskuva voi sisältää esimerkiksi Linuxin perustyökalut.
Kerroksia voidaan tallentaa välimuistiin ja jakaa kuvien kesken, mikä lyhentää
niiden rakennusaikaa ja vähentää tallennustilan käyttöä.

Docker-kontti on kuvan käynnissä oleva ilmentymä. Suurissa palveluissa Dockeria
käytetään palveluiden dynaamiseen replikointiin ja skaalaamiseen useisiin,
yleensä hajautettuihin kontteihin.

Docker-kuvat viedään usein rekisteriin, kuten `docker.io`-palveluun, josta kuvia
tarvitsevat voivat löytää ja ottaa niitä käyttöön. Tällä kurssilla rekisteriä ei
käytetä. Sen sijaan kurssipalvelin rakentaa Docker-kuvat opiskelijoiden
Git-repositorioiden perusteella ja suorittaa ne palvelimella, jotta
asiakastoteutukset voivat muodostaa niihin yhteyden.

Alla oleva kuva näyttää kurssipalvelimemme Docker-järjestelyn, ja kuinka pyyntö
`run-docker` rajapintaan ottaa ensin git-kloonin opiskelijan repositoriasta, ja
rakentaa sen jälkeen Docker-kuvan repositoriossa olevan `Dockerfile`:n pohjalta
ja ajaa kontainerin. Lisätietoja rajapinnan toiminnasta on tehtäväkuvauksessa
tämän sivun lopussa.

![Kurssipalvelimen Docker-asetelma](/images/server-docker.svg){: width="90%" .center-img }

Useimmissa järjestelmissä helpoin tapa asentaa Docker paikallisesti on asentaa
[Docker Desktop](https://docs.docker.com/desktop/), joka on saatavilla
Windowsille, Mac:lle ja Linuxille. Docker Desktop tarjoaa graafisen
käyttöliittymän paikallisen järjestelmän konttien hallintaan sekä jäljempänä
käsiteltävät komentorivityökalut.

### Docker-kuvan rakentaminen

Alla on esimerkki `Dockerfile`-tiedostosta. Sen **rakennusosa (builder)**
kääntää binäärimuotoisen palvelinsovelluksen Rust-lähdekoodista, ja **ajo-osa
(runtime)** suorittaa palvelinsovelluksen sekä avaa TCP-portin 1234 ulkopuolelta
tuleville yhteyksille. Esimerkkiä pitäisi olla helppo soveltaa omaan projektiisi
sopivaksi.

```dockerfile
# Käännä Rust-palvelinsovellus rakennusosassa.
FROM rust:1.96 AS builder

# Aseta kontin sisäinen työhakemisto.
WORKDIR /usr/src/my-server-app

# Kopioi lähdekoodi paikalliselta koneelta konttiin.
COPY ./ ./

# Käännä tuotantoversio (parempi suorituskyky) palvelimesta kontin sisällä.
# Tässä tapauksessa palvelin sijaitsee Rust-työtilan erillisessä
# 'server'-paketissa. Asiakasohjelmaa ei tarvitse sisällyttää konttiin.
RUN cargo build -p server --release


# Suorita käännetty palvelin pienemmässä ajonaikaisessa kuvassa.
FROM debian:bookworm-slim AS runtime

WORKDIR /usr/local/bin

# Kopioi käännetty palvelinbinääri rakennusosasta ajonaikaiseen osaan.
COPY --from=builder /usr/src/pronets-demo-chat/target/release/server ./server

# Avaa portti, jota palvelin kuuntelee.
EXPOSE 1234

# Määritä komento, joka suoritetaan kontin käynnistyessä.
CMD ["./server", "--port", "1234"]
```

Dockerfile-tiedoston rakennusosa voidaan suorittaa komentoriviltä esimerkiksi
seuraavalla komennolla:

```bash
docker build -t my-server-app .
```

Valitsin `-t my-server-app` antaa rakennetulle kuvalle paikallisen nimen.
Komennon lopussa oleva `.` käskee Dockeria käyttämään nykyistä hakemistoa
rakennuskontekstina.

Tämän jälkeen kontti voidaan käynnistää seuraavasti:

```bash
docker run -d -p 1234:1234 --name my-server-app my-server-app:latest
```

Valitsin `-d` käynnistää palvelimen taustalle. Jos palvelin halutaan pitää
edustalla esimerkiksi kehittämistä ja debugausta varten, valitsin
`-d` voidaan jättää pois. Valitsin `-p 1234:1234` julkaisee kontin TCP-portin
1234 isäntäkoneen portissa 1234, jolloin ulkoiset sovellukset voivat muodostaa
siihen yhteyden.

### Dockerin käyttäminen

Vaikka Docker Desktopissa on graafinen käyttöliittymä paikallisen järjestelmän
konttien hallintaan, seuraavassa esitellään lyhyesti hyödyllisimmät
komentorivitoiminnot.

Kaikki käynnissä olevat kontit voidaan luetella näin:

    docker ps

Konttiluettelossa näkyvät kontin tunniste ja nimi, joiden avulla komentoja
voidaan kohdistaa eri kontteihin.

Kontin lokit näytetään näin:

    docker logs container-name

Lokit sisältävät tulosteen, jonka kontin sisällä toimivat ohjelmat ovat
kirjoittaneet vakiotulosteeseen. Pitkään toimivan palvelimen lokit kasvavat ajan
myötä. Siksi etenkin kehitys ja debugausvaiheessa seuraava komento voi olla
hyödyllinen:

    docker logs --tail 100 -f container-name

Komento näyttää aluksi vain tulosteen 100 viimeistä riviä, minkä jälkeen pääte
jatkaa tulostevirran seuraamista, kunnes toiminto keskeytetään
näppäinyhdistelmällä Ctrl-C.

Käynnissä oleva kontti pysäytetään näin:

    docker stop container-name

Kontin toimintaa ja tilaa voidaan tutkia tarkemmin myös suorittamalla komentoja
käynnissä olevan kontin sisällä. Esimerkiksi nykyisen tiedostojärjestelmän
sisältö voidaan luetella seuraavasti:

    docker exec container-name ls /app-dir

Kontin sisällä voidaan käynnistää myös komentorivi-istunto tarkempaa
tutkimista varten:

    docker exec -it container-name /bin/sh

Valitsin `-i` pitää kontin vakiosyötteen avoinna, joten sille voidaan kirjoittaa
syötettä. Valitsin `-t` varaa kontille pseudoterminaalin, jolloin istunto toimii
vuorovaikutteisen päätteen tavoin.

## Rust-projektin rakenne ja työtilat

Kurssin edellisessä osiossa mainittiin jo lyhyesti Rust-projektin moduulit ja
paketit. Seuraavassa käsitellään ja ehdotetaan projektirakennetta, jossa
Rustilla toteutettu asiakas–palvelinprojekti järjestetään kahdeksi paketiksi
yhteiseen työtilaan yhteen git-repositorioon. Kummallakin paketilla on oma
`Cargo.toml`-tiedostonsa, ja niistä käännetään erilliset binääritiedostot.
Joissakin projekteissa voi esimerkiksi olla paketteja, jotka sisältävät
suuremman ohjelmiston tarvitsemia kirjastoja, sekä paketteja, jotka
sisältävät näitä kirjastoja käyttäviä ajettavia ohjelmia.

Asiakas–palvelinprojektia varten on laadittu tätä rakennetta esittelevä
**[project-template](https://github.com/PasiSa/pronets/tree/main/examples/project-template)**-esimerkki,
jota voit halutessasi käyttää oman työsi pohjana. Projektipohjassa viitataan
käsitteisiin, joita ei vielä ole käsitelty, kuten säikeet (threads) tai
_tokio_-kirjasto, mutta näistä ei kannata vielä murehtia. Tämän viikon tehtävän
asiakasosuuden voit toteuttaa yksinkertaisena komentorivipohjaisena ohjelmana,
tai voit muokata esimerkkipohjan asiakasta tarpeen mukaan.

Vaikka kurssiprojektimme on melko pieni, asiakas- ja palvelintoteutukset
kannattaa monesta syystä jakaa kahdeksi erilliseksi paketiksi samaan työtilaan,
etenkin jos asiakkaalle halutaan tehdä graafinen käyttöliittymä. Esimerkin
käyttöliittymä on toteutettu
**[Slint](https://slint.dev/)**-käyttöliittymäkirjasolla. Sen tarvitsemat
kirjastot (crate) täytyy lisätä asiakastoteutuksen
[Cargo.toml](https://github.com/PasiSa/pronets/blob/main/examples/project-template/client/Cargo.toml)-tiedostoon
riippuvuuksina. Näistä kirjastoista ei kuitenkaan olisi hyötyä palvelimessa,
joten niitä ei tarvitse kääntää osaksi palvelinohjelmaa eikä palvelimellamme
suoritettavaa Docker-kuvaa. Vastaavasti palvelinpaketti voi tarvita kirjastoja,
joita asiakas ei tarvitse. Kahden toisiinsa liittyvän sovelluksen pitäminen
samassa työtilassa yhdistää ne kuitenkin toisiinsa ja mahdollistaa yhteisen
koodin, kuten viestien parsimiseen käytettävän koodin, jakamisen samassa
Git-repositoriossa.

Asiakasesimerkki näyttää myös, kuinka pakettiin voidaan sisällyttää ulkoisia
kirjastoja eli (crate) lisäämällä ne
[Cargo.toml](https://github.com/PasiSa/pronets/blob/main/examples/project-template/client/Cargo.toml)-tiedostoon.
Tällöin ne noudetaan automaattisesti Rustin crate-rekisteristä
([crates.io](https://crates.io/)). Käytettävä kirjastoversio kannattaa
määrittää. Jotkin suuremmat kirjastot on jaettu erikseen valittaviin
ominaisuuksiin (_features_), joiden avulla lopullisen binääritiedoston kokoa
voidaan pienentää jättämällä tarpeettomat ominaisuudet pois.

<div class="assignment-frame" markdown="1">

## Tehtävä #3

Aloitamme nyt verkkosovellusprojektin kehittämisen kurssin alussa luodussa
Git-repositoriossa. Asiakas- ja palvelintoteutukset kannattaa sijoittaa samaan
repositorioon ja samaan Rust-työtilaan erillisiksi paketeiksi edellä kuvatulla
tavalla.

**Osa 1**: Toteuta yksinkertainen palvelin, joka kuuntelee saapuvia yhteyksiä
sinulle varatussa portissa. Tässä vaiheessa palvelin tunnistaa vain yhden
viestityypin: testiviestin **TST**, jolla voidaan tarkistaa, että palvelin on
käynnissä ja vastaa pyyntöihin.

Yleinen kurssitoteutuksissa käytetty viestirakenne, jota myös TST-viesti
noudattaa on seuraavanlainen:

- Viesti alkaa **32-bittisellä etumerkittömällä kokonaisluvulla (u32)**, joka
  ilmaisee **viestin pituuden**. Kokonaisluvun täytyy olla verkkojärjestyksen
  mukaisessa **big-endian**-tavujärjestyksessä. Katso edellisestä moduulista,
  miten tämä tehdään. Pituus kattaa koko viestin, myös itse pituuskentän ja
  tunnistenumeron.

- Seuraavana on toinen **32-bittinen etumerkitön kokonaisluku**, joka sisältää
  **viestin tunnisteen** verkon tavujärjestyksessä. Myöhemmin tunnisteen avulla
  voidaan yhdistää vastaukset erilaisiin pyyntöviesteihin.

- Tämän jälkeen tulee viestityyppi "TST", välilyönti ja vapaamuotoinen sisältö.

Kun palvelin vastaanottaa tällaisen viestin, sen täytyy lähettää sama
viestisisältö takaisin asiakkaalle käyttäen samaa pituutta ja viestin
tunnistetta.

**Osa 2**: Toteuta myös asiakasohjelma, joka lähettää TST-viestin, jotta voit
testata palvelinta.

**Osa 3**: Lisää projektiisi palvelimen rakentava ja käynnistävä
**Dockerfile**. Tiedoston täytyy sijaita Git-repositoriosi juuressa ja sen
nimen täytyy olla `Dockerfile`, jotta kurssipalvelimemme löytää sen
Docker-kuvan rakentamista varten.

Kun Dockerfile on luotu ja olet testanut palvelimen toimintaa paikallisesti
esim. Docker Desktopilla, vie työsi Git-palvelimella olevaan repositorioosi.

**Osa 4**: Palvelinohjelmasi rekisteröidään kurssipalvelimelle lähettämällä HTTP
POST-pyyntö palvelimelle **pronets1.dice.aalto.fi** porttiin 80. HTTP-päätepiste on
`POST /run-docker`. POST-pyynnön rungon täytyy olla JSON-koodattu ja sisältää
seuraavat kentät:

- **"name"**: Käyttäjänimi, jota haluat käyttää palvelimella. Tämän täytyy olla
  aiemmin ilmoittamasi käyttäjänimi.
- **"git-repo"**: Kurssin tehtäviin ja projektiin käyttämäsi Git-repositorion
  URL-osoite.
- **"ports"**: Portin tai porttien numerot, joissa palvelin kuuntelee yhteyksiä.
  Tässä vaiheessa portteja on vain yksi, ja se annetaan merkkijonona.
- **"protocol"**: Protokollan tunniste. Sen arvon täytyy tässä vaiheessa olla
  `base-1`. Protokollia käsitellään tarkemmin seuraavassa moduulissa.

Huomaa, että rakenne on sama kuin edellisessä tehtävässä toteutetulla
`/fetch-git`-päätepisteellä, mutta mukana on pari lisäkenttää.

Toteuta ohjelma, joka lähettää HTTP-pyynnön ja odottaa vastausta. Toiminto voi
olla myös esimerkiksi osa asiakasohjelmasi koodia. Kun kurssipalvelin
vastaanottaa pyynnön, se hakee koodisi Git-repositoriosta sekä rakentaa ja
käynnistää palvelimesi Dockerfile-tiedoston avulla. Huomaa, että tämä vie aikaa,
jopa minuutteja. Jos kaikki onnistuu ja saat lopulta vastauksen "OK",
palvelinkoodisi pitäisi olla käynnissä osoitteessa `pronets1.dice.aalto.fi` ja
kuunnella yhteyksiä ilmoittamassasi portissa.

Kurssin pääpalvelin lähettää TST-viestin 60 sekunnin välein kaikille
käynnistetyille konteille. Näet kaikki rekisteröidyt palvelimet, niiden portit
ja viimeisimmän TST-viestin tuloksen osoitteessa
**[https://pronets1.dice.aalto.fi/](https://pronets1.dice.aalto.fi/)**.

Kirjoita lyhyt raportti, jossa dokumentoit etenemisesi edellä kuvatuissa
vaiheissa. Kerro jokaisesta askelesta miten lähestyit ongelmaa, mitä haasteita
kohtasit, ja miten sait ne ratkaistua. Löydätkö oman konttisi pavelimen
containers-näkymästä, ja näkyykö sen kohdalla teksti "OK"?

Sisällytä raporttiin jälleen myös seuraavat tiedot:

- Kuinka paljon aikaa käytit tehtävään?
- Mikä tehtävässä oli helppoa tai vaikeaa?
- Mitä työkaluja tai muita tietolähteitä käytit? Varsinkin jos käytit
  tekoälyavustimia, kerro miten käytit niitä ja olivatko ne hyödyllisiä.

</div>
