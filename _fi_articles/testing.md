---
title: Testaus ja havainnoitavuus
lang: fi-FI
translation_key: testing
---

Tässä moduulissa tarkastellaan erilaisia tapoja analysoida ohjelmiston toimintaa.
Pienissä ohjelmissa virheiden selvittämiseen riittävät `println!`-tulostukset tai
debuggerityökalut, mutta ohjelmiston kasvaessa sen analysointiin tarvitaan
kehittyneempiä keinoja. Erityisesti silloin, kun verkkopalvelin otetaan käyttöön ja
sen on tarkoitus toimia yhtäjaksoisesti pitkiä aikoja, on tärkeää pystyä
seuraamaan sen suorituskykyä ja mahdollisia poikkeamia odotetusta toiminnasta.
Näin järjestelmän ylläpitäjä voi reagoida ongelmiin ja ryhtyä tarvittaviin
korjaustoimiin.

<div class="objectives-frame" markdown="1">

**Tämän moduulin tavoitteet:**

- **Opit jakamaan toteutuksen moduuleihin** ylläpidettävyyden parantamiseksi.
  Tämä helpottaa myös testauksen järjestämistä.

- **Ymmärrät testauksen eri muodot**, erityisesti **yksikkötestit**,
  **integraatiotestit** ja **päästä päähän -testit**, sekä sen, miten Rust tukee
  niitä.

- **Opit käyttämään lokitus- ja jäljitystyökaluja** osana Rust-ohjelmaasi
  toteutuksen toiminnan selvittämiseksi.

- **Ymmärrät erilaiset mittarit** ja sen, miten niiden avulla voidaan analysoida
  käytössä olevan ohjelmiston suorituskykyä ajan kuluessa.

- **Saat yleiskuvan valvontatyökaluista** ja siitä, miten ne hyödyntävät
  ohjelmiston keräämiä mittareita.

</div>

## Koodin jakaminen moduuleihin

Hyvä ohjelmistosuunnittelun periaate on pitää suoritettavan ohjelman `main.rs`
tiedosto melko lyhyenä ja jakaa suurin osa toiminnallisuudesta moduuleihin, jotka
sijaitsevat erillisissä tiedostoissa `src/`-hakemistossa. Moduulit tulee myös
luetella `main.rs`-tiedoston tai kirjastopaketeissa käytettävän `lib.rs`-tiedoston
alussa tai lopussa esimerkiksi seuraavasti:

```rust
mod channel;
mod client;
mod metrics;
```

Tämä voisi olla verkkopalvelimen rakenne, jossa **client**-moduuli kokoaa yhteen
yksittäiseen asiakkaaseen liittyvän logiikan, **channel**-moduuli
keskustelukanavaan liittyvän logiikan ja **metrics**-moduuli palvelimen mittarien
hallinnan. Suuremmissa ohjelmistoissa moduulit voidaan järjestää hierarkiaksi
`src/`-hakemiston alihakemistoihin. Rust-kirjan [luvussa
7.5](https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html)
käsitellään tätä tarkemmin.

Lisäksi, silloin kun se sopii tarkoitukseen, hyvä periaate olisi kapseloida
moduuliin liittyvä toiminnallisuus ja tarvittavat tiedot tietorakenteeksi joka
vastaa moduulin nimeä, ja sitä käsitteleviksi metodeiksi. Esimerkiksi
**client**-moduulin tapauksessa voisimme määritellä `Client`-tietorakenteen ja
sen metodit seuraavaan tapaan (kuten lyhyesti jo moduulissa 2 käytiin läpi):

```rust
/// Kuvaa yhtä asiakasyhteyttä palvelimeen.
pub struct Client {
    stream: Option<TcpStream>,
    handle: ClientHandle,
    username: Option<String>,
    // ...ja luultavasti muita kenttiä...
}

impl Client {
    pub fn new(stream: TcpStream) {
        // asiakasolion muodostaminen tähän
    }

    pub fn process_msg(&mut self, message_id: u32, length: u32) -> std::io::Result<()> {
        // toteutus tähän
    }
    // ...muita funktioita...
}
```

Tietuetta ja sen metodeja voidaan käyttää muista moduuleista lisäämällä
moduulin alkuun seuraavanlainen `use` - komento (`crate::`-etuliitteen voi
jättää pois nimiavaruuden juuressa olevassa `main.rs`-tiedostossa):

```rust
use crate::client::Client;
```

Huomaa, että tietue ja sen metodit on määriteltävä **julkisiksi** (`pub`), jotta
niitä voidaan käyttää muista moduuleista.

## Testaus

Testeillä tarkistetaan ohjelmiston toimintaa eri tasoilla. **Yksikkötestit**
testaavat yksittäisiä funktioita tai pieniä logiikan osia erillään muusta
ohjelmistosta. **Integraatiotestit** tarkistavat, että useat komponentit
toimivat oikein yhdessä. **Päästä päähän -testit** testaavat koko sovellusta
käyttäjän tai asiakkaan näkökulmasta esimerkiksi muodostamalla yhteyden
käynnissä olevaan palvelimeen, lähettämällä pyynnön ja tarkistamalla vastauksen.
Testien lisääminen ja olemassa olevien testien parantaminen tulisikin nähdä
osana ohjelmistokehitystä, kun uusia toimintoja toteutetaan. Aina kun
ohjelmistoon lisätään uusi ominaisuus, on hyvä tapa kirjoittaa sille myös
testit.

Rust tukee automatisoituja yksikkö- ja integraatiotestejä. Kun ohjelmisto
sisältää testejä, komento `cargo test` käy läpi projektin testit ja suorittaa
ne. Ennen jokaista git-committia on hyvä varmistaa, että kaikki testit menevät
läpi, ja tehdä tarvittavat korjaukset, jos näin ei ole. Rust-kirjan [luvussa
11.1](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html) kuvataan
tarkemmin testien kirjoittamista.

Yleiset Git-palvelut, kuten GitLab ja GitHub, tukevat automatisoituja
työnkulkuja ("workflows"), jotka voidaan suorittaa esimerkiksi jokaisen
push-tapahtuman tai yhdistämispyynnön (merge request) yhteydessä.
Yhdistämispyynnöllä kehittäjä voi ehdottaa arvioitavaksi koodimuutoksia
päähaaraan projekteissa joissa on mukana useita kehittäjiä. Näiden työnkulkujen
avulla voidaan toteuttaa **jatkuva integraatio (Continuous Integration, CI)**
eli esimerkiksi suorittaa testit ja tarkistaa koodin muotoilu automaattisesti,
ja edellyttää niiden läpimenemistä ehtona muutosten hyväksymiselle.

### Assert-makrot

Assert-makrot tarkistavat, että ehto tai tulos vastaa odotettua. Jos tarkistus
epäonnistuu, se aiheuttaa paniikin (eli ohjelman keskeytymisen), jonka
seurauksena testi epäonnistuu. Huutomerkki (`!`) ilmaisee, että kyseessä on
Rust-makro. Yleisimmät tarkistukset ovat:

- `assert!(condition)`: tarkistaa, että totuusarvoinen ehto on tosi, esimerkiksi
  `assert!(!encoded.is_empty())`.
- `assert_eq!(actual, expected)`: tarkistaa, että kaksi arvoa ovat yhtä suuret,
  esimerkiksi `assert_eq!(encoded.len(), 9)`.
- `assert_ne!(actual, unexpected)`: tarkistaa, että kaksi arvoa ovat erisuuret,
  esimerkiksi `assert_ne!(encoded.len(), 0)`.

Yhtäsuuruutta ja erisuuruutta tarkistavat makrot näyttävät molemmat arvot, jos
tarkistus epäonnistuu, mikä auttaa ongelman selvittämisessä. Kaikille kolmelle
makrolle voi myös antaa valinnaisen muotoillun viestin, joka selittää
epäonnistumisen syyn, esimerkiksi
`assert_eq!(encoded.len(), 9, "unexpected length: {}", encoded.len())`.

### Yksikkötestit

**Yksikkötestit** tarkistavat yleensä, että yksittäinen funktio tai
toiminnallisuus toimii odotetusti. Rustissa yksikkötestit sijoitetaan saman
ohjelmamoduulin (eli `*.rs`-tiedoston) loppuun kuin testattava toteutus.
Ohjelman kasvaessa se kannattaa jakaa erillisiin loogisiin moduuleihin, mikä
helpottaa myös testien suunnittelua.

Seuraavassa on esimerkki yksinkertaisesta funktiosta, joka lisää pituuskentän
viestin alkuun, sekä sitä seuraavasta yksikkötestistä:

```rust
/// Funktio, joka lisää pituuskentän argumenttina annetun viestin alkuun
pub fn encode_message(payload: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(4 + payload.len());
    result.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    result.extend_from_slice(payload);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Yksikkötesti, joka tarkistaa, että funktio lisää oikean pituusotsakkeen
    #[test]
    fn encodes_length_before_payload() {
        let encoded = encode_message(b"hello");

        assert_eq!(&encoded[..4], &[0, 0, 0, 5]);
        assert_eq!(&encoded[4..], b"hello");
    }
}
```

Ohjelman testiosio alkaa merkinnällä `#[cfg(test)]`, joka kertoo
Rust-kääntäjälle, että seuraava koodi käännetään vain testejä varten. Testit
sijoitetaan nykyisen ohjelmamoduulin `tests`-alimoduuliin. `#[test]` on Rustin
sisäänrakennettu attribuutti, joka kertoo Rustin testisuorittajalle, että
funktio tulisi sisällyttää `cargo test` -komennolla suoritettaviin testeihin.

### Integraatiotestit

**Integraatiotesti** tarkistaa, että useat ohjelmistokomponentit ja funktiot sekä
niiden julkiset rajapinnat toimivat oikein yhdessä tietyn ominaisuuden
toteuttamiseksi. Siksi integraatiotestit eivät loogisesti kuulu osaksi jotakin
ohjelmamoduulia yksikkötestien tapaan, vaan ne sijoitetaan lähdekoodissa erilleen.
Rustissa integraatiotestit sijoitetaan projektin ylätason hakemistossa
olevaan erilliseen `tests`-hakemistoon varsinaisen koodin sisältävän
`src`-hakemiston rinnalle.

Esimerkki
**[integration-test](https://github.com/PasiSa/pronets/tree/main/examples/integration-test/)**
esittää yksinkertaisen palvelintoteutuksen, joka on jaettu kahteen moduuliin.
**server**-moduuli sisältää funktion `handle_connection`, joka ottaa vastaan
TCP-yhteyden omistajuuden, lukee viestin ja lähettää sen takaisin, sekä sulkee
lopuksi yhteyden (koska TCP-pistoke poistuu näkyvyysalueelta funktion lopussa).
Luku- ja kirjoitusoperaatiot tehdään erillisessä **protocol**-moduulissa, jota
palvelin kutsuu. Integraatiotestit edellyttävät myös, että projektissa on
`lib.rs`-tiedosto, joka liittää moduulit samaan käännösyksikköön (crate), vaikka
projektissa olisi myös `main.rs`. Huomaa, että tämä esimerkki toimii kirjastona:
siinä ei ole `main()`-funktiota eikä **main.rs**-tiedostoa.

Tiedostossa `tests/roundtrip.rs` on yksi integraatiotesti, joka avaa
palvelinpistokkeen erillisessä säikeessä (joita käsitellään seuraavassa
moduulissa) ja yhdistää sitten asiakaspistokkeen siihen. Näin testataan
`handle_connection`-funktiota ja kahden moduulin yhteistoimintaa. Palvelimen ja
asiakkaan toteutusten jakaminen erillisiin säikeisiin on melko yleinen ratkaisu
integraatiotesteissä, joissa testataan kommunikaatioita verkon yli.

Voit kokeilla esimerkin testiä suorittamalla komennon `cargo test`. Huomaa, että
komentoa `cargo run` ei voi käyttää, koska esimerkissä ei ole `main()`-funktiota.

Kun projekti koostuu useita paketteja sisältävästä työtilasta, kuten
asiakas/palvelinprojekteissamme, kummallakin paketilla on omat
integraatiotestinsä. Tällöin `tests`-hakemisto on sijoitettava erikseen
kumpaankin pakettiin.

### Päästä päähän -testit

**Päästä päähän -testit** testaavat koko sovelluksen ulkoista toimintaa.
Jos asiakkaalla on graafinen käyttöliittymä, sen toiminnan automaattinen
analysointi voi olla vaikeaa ilman erityisiä työkaluja. On kuitenkin mahdollista
kehittää testiasiakas, joka simuloi erilaisia asiakastilanteita palvelimen kanssa
viestittäessä ja tulostaa tulokset komentoriville.

Projektissamme voisimme ensin käynnistää varsinaisen palvelinohjelman tai
Docker-kontin ja toteuttaa komentorivillä toimivan testiasiakkaan, joka lähettää
erilaisia viestisarjoja käyttäen sekä kelvollisia että virheellisiä viestejä.
Asiakas tarkistaa, että palvelimen vastaukset ovat odotetun mukaisia, ja raportoi
tulokset komentorivillä.

Oletusarvoisesti suoritettava Rust-pakkaus suorittaa `src`-hakemistossa olevan
**main.rs**-lähdetiedoston `main()`-funktion. Vaihtoehtoisia suoritettavia
ohjelmia voi sijoittaa `src`-hakemiston `bin`-alihakemistoon, ja kullakin niistä
on oma `main()`-funktionsa. Näin voidaan toteuttaa esimerkiksi automaattisen
testisarjan suorittava testiasiakas, kun varsinainen vuorovaikutteinen (ja
mahdollisesti graafinen) asiakasohjelma on **client**-paketin
**main.rs**-tiedostossa.

Kun samassa pakkauksessa on useita suoritettavia ohjelmia, ohjelman nimi on
määritettävä, kun se käynnistetään `cargo`-komennolla. Tiedostossa **main.rs**
suoritettavalla ohjelmalla on sama nimi kuin pakkauksella. Jos client-paketissa
olisi `main.rs`-tiedoston lisäksi `bin/testclient.rs`, voisimme suorittaa
komennon

    cargo run -p client --bin client

käynnistääksemme varsinaisen asiakasohjelman **main.rs**-tiedostosta
(`-p`-valitsin kertoo, että käytämme client-pakettia emmekä palvelinpakettia),
tai komennon

    cargo run -p client --bin testclient

suorittaaksemme päästä päähän -testit.

## Lokitus ja jäljitys

Tavallinen ja suoraviivainen tapa seurata ohjelman suoritusta on tulostaa
ohjelman tapahtumia konsoliin `println!`-makrolla. Palvelimen kasvaessa on
hyödyllistä voida hallita kirjattavien tietojen yksityiskohtaisuutta ja
tarkentaa, mitä yhteyttä tai pyyntöä kukin viesti koskee. **Lokitus** tallentaa
suorituksen aikaisia tapahtumia, kuten palvelimen käynnistymisen, asiakkaan
yhdistämisen tai pyynnön epäonnistumisen.

### Lokitustasot

Kehityksen aikana haluamme usein yksityiskohtaista tietoa ohjelman
suorituksesta. Kun ohjelma on tuotantokäytössä ja siinä voi olla paljon
kommunikointiliikennettä, jokaisen asiakkaan jokaisen vaiheen kirjaaminen
tuottaa liikaa tulostetta. Lokitustasojen avulla voimme valita sopivan
yksityiskohtaisuuden tason. Seuraavassa käytämme **tracing**-kirjastoa, joka
tarjoaa seuraavat makrot eri vakavuustasojen lokitukseen (listattu vakavimmasta
lievimpään):

- **ERROR**, `error!`: Huomiota vaativa virhe, kuten se, ettei palvelin pysty
  sitomaan kuuntelupistokettaan tai käyttämään tarvittavaa tallennustilaa.
- **WARN**, `warn!`: Odottamaton tilanne, josta palvelin pystyy toipumaan,
  kuten virheellisen protokollaviestin hylkääminen samalla, kun muiden asiakkaiden
  palveleminen jatkuu.
- **INFO**, `info!`: Tärkeät normaalit tapahtumat, kuten palvelimen
  käynnistyminen tai sammuminen. Myös esimerkiksi yhteyden alkaminen ja
  päättyminen voi olla hyödyllistä kirjata tällä tasolla projektissamme.
- **DEBUG**, `debug!`: Kehityksen aikana hyödylliset yksityiskohdat, kuten
  käsiteltävän viestin tyyppi ja tunniste.
- **TRACE**, `trace!`: Suorituksen tarkat yksityiskohdat, kuten eteneminen
  jäsentämisen yksittäisissä vaiheissa.

Vakavuustaso tulee valita sen mukaan, miten tapahtuma vaikuttaa sovellukseen.
Asiakkaan virheellinen syöte ei välttämättä tarkoita palvelimen virhettä, ja
tavallinen asiakasyhteyden katkeaminen on yleensä odotettua. Usein toistuvat
tapahtumat, kuten jokainen onnistuneesti käsitelty viesti, kuuluvat tavallisesti
DEBUG- tai TRACE-tasolle.

### Jäljityksen käyttöönotto

Tässä esitelty **tracing**-kirjasto koostuu kahdesta osasta. `tracing`-paketti
tuottaa tapahtumat ja jäljitysjaksot (joista kerrotaan pian lisää). Jäljityksen
**tilaaja** (_subscriber_) vastaanottaa tapahtumat, suodattaa niitä ja
määrittää, miten ne tallennetaan. `tracing-subscriber`-paketti tarjoaa tilaajan,
joka tuottaa muotoiltua tekstitulostetta. Jäljitystuki otetaan käyttöön
lisäämällä seuraavat riippuvuudet palvelinpaketin `Cargo.toml`-tiedoston
`[dependencies]`-osioon:

```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

Jäljitys otetaan käyttöön `main()`-funktion alussa ennen suurinta osaa muusta
ohjelmalogiikasta. Tässä on lyhyt esimerkki:

```rust
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

fn main() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    info!("server starting");
    debug!("configuration loaded");
}
```

Edellä oleva toteutus lukee `RUST_LOG`-ympäristömuuttujan ja käyttää INFO-tasoa, jos
ympäristömuuttujaa ei ole asetettu. INFO-tasolla lokiin sisällytetään INFO-,
WARN- ja ERROR-tapahtumat. Se on yleensä hyvä vaihtoehto, kun palvelinta ajetaan
tuotannossa (tai kurssipalvelimellamme). `env-filter`-ominaisuus ja
`.with_env_filter(...)`-asetus ottavat käyttöön ympäristömuuttujaan perustuvan
suodatuksen. Lisätietoja löytyy [tracing-subscriber
dokumentaatiossa](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/).

Kun palvelinta ajetaan komentoriviltä, ympäristömuuttuja ja lokitustaso voidaan
asettaa esimerkiksi seuraavasti. Kullakin tasolla näytetään myös kaikki sitä
vakavammat viestit, joten DEBUG-tason lokitus voi tuottaa paljon tulostetta, jos
sitä on käytetty ohjelmassa runsaasti.

```sh
RUST_LOG=info cargo run -p server
RUST_LOG=debug cargo run -p server
```

Ympäristömuuttuja voidaan asettaa Docker-kontille lisäämällä seuraavanlainen
rivi `Dockerfile`-tiedostoon ennen palvelimen käynnistävää CMD-riviä, jolloin
lokitustaso voidaan valita tarpeen mukaan:

```
ENV RUST_LOG=info
```

**Tiedoksi:** Kurssipalvelimella `run-docker`-POST-pyyntöön voi lisätä
valinnaisen attribuutin ympäristömuuttujien asettamista varten. Sen avulla voit
esimerkiksi muuttaa lokitustasoa tilapäisesti virheenjäljitystä varten ilman
että `Dockerfile`:ä tarvitsee muuttaa. JSON-pyyntöön lisättävän attribuutin nimi
on `env`, ja sen arvoksi annetaan merkkijonotaulukko määriteltävistä
ympäristömuuttujista, esimerkiksi: `["RUST_LOG=debug", "SECRET_VALUE=xyz"]`.

### Tapahtumat

**Tapahtuma** tallentaa jotakin, joka tapahtui tietyllä hetkellä ohjelman
suorituksen aikana. Luettavan viestin lisäksi se voi sisältää nimettyjä
**kenttiä**. Esimerkiksi seuraava funktio kirjaa tietoja jo puretusta viestistä:

```rust
fn record_message(connection_id: u64, message_id: u32, message_type: &str) {
    tracing::debug!(
        connection_id,
        message_id,
        message_type,
        "message received"
    );
}
```

Vaikka makroja voi käyttää kutsulle annettujen parametrien muotoiluun samaan
tapaan kuin `println!`- tai `format!`-makroja, **tracing**-kirjasto mahdollistaa
myös valittujen muuttujien sisällyttämisen tulosteeseen muodossa
`muuttujan_nimi=arvo`. Yllä näin tehdään muuttujille `connection_id`,
`message_id` ja `message_type`. Esimerkissä oletetaan, että jokaiselle
hyväksytylle yhteydelle annetaan numeerinen tunniste lokitusta varten.

Voit jättää `tracing::`-etuliitteen pois lokituskutsuista lisäämällä tiedoston
alkuun seuraavan rivin:

```rust
use tracing::*;
```

Lisätietoja muotoilusta ja muista yksityiskohdista on [tracing-makrojen
dokumentaatiossa](https://docs.rs/tracing/latest/tracing/#using-the-macros).

### Rakenteinen jäljitys ja jäljitysjaksot

Kun useat asiakkaat viestivät palvelimen kanssa, niiden tapahtumat lomittuvat.
**Rakenteinen jäljitys** lisää tapahtumiin kontekstia **jäljitysjaksojen**
(_span_) avulla. Ne ovat nimettyjä toimintoja, joilla on alku ja loppu. Yhteyden
jäljitysjakso voi sisältää kyseisen yhteyden tapahtumat sekä kullekin pyynnölle
oman sisäkkäisen jäljitysjakson. Jäljitysjakson kentät yksilöivät toiminnon, ja
ne sisällytetään jakson sisäisiin tapahtumiin, joten niitä ei tarvitse toistaa
jokaisessa lokitapahtumassa.

Attribuutti `#[tracing::instrument]` luo jäljitysjakson funktion ympärille.
Esimerkiksi yhteyden käsittelijä saattaisi kutsua seuraavaa funktiota kerran
jokaista dekoodattua viestiä kohden:

```rust
#[tracing::instrument(
    name = "message",
    level = "debug",
    skip(payload),
    fields(payload_bytes = payload.len())
)]
fn process_message(message_id: u32, message_type: &str, payload: &[u8]) {
    tracing::debug!("processing message");
    // Tarkista ja käsittele viesti tässä.
    tracing::debug!("message processing complete");
}
```

Oletusarvoisesti attribuutti tallentaa kaikki funktion argumentit kentiksi.
`skip(payload)` jättää viestin sisällön pois lokista, kun taas `payload_bytes`
tallentaa viestin koon. Muut funktion parameterit, jotka yksilöivät viestin,
sisällytetään jäljitysjakson tapahtumiin. Lisää asetuksia esitellään
[instrument-dokumentaatiossa](https://docs.rs/tracing/latest/tracing/attr.instrument.html).

### Mitä kannattaa kirjata?

Lokituksessa kannattaa huomioida seuraavia asioita:

- Kannattaa käyttää yhdenmukaisia kenttien nimiä eri lokiviesteissä tapahtumien
  ja niiden keskinäisten suhteiden analysoinnin helpottamiseksi
- Kannattaa tallentaa riittävästi tietoa virheen ymmärtämiseksi, kuten tietoa
  epäonnistuneesta toiminnosta ja sen virhekoodi tai virhekuvaus. Virhe
  kannattaa kirjata näiden tietojen kanssa siinä kohdassa, jossa se käsitellään,
  sen sijaan että kirjaisit sen toistuvasti jokaisessa funktiossa, jonka kautta
  virhetieto kulkee.
- **Tärkeää:** Älä tallenna salasanoja, autentikointitunnuksia (käsitellään
  myöhemmin) tai muita yksityisiä tietoja (kuten viestien sisältöä). Muista,
  että jäljitysjaksojen yhteydessä automaattisesti tallennetut funktion
  argumentit voivat myös sisältää tällaisia arvoja. On hyvä käyttää
  `skip(...)`-toimintoa jäljitysjaksojen yhteydessä silloin kun se on tarpeen.
  Kannattaa erityisesti muistaa, että kurssipalvelimellamme konttien lokit
  näkyvät kaikille.

## Metriikat

Vaikka hyvin suunnitellut lokit ja jäljitykset antavat yksityiskohtaista tietoa
ohjelmiston toiminnasta, pelkästään niiden perusteella on vaikea muodostaa
kokonaiskuvaa palvelimen kuormituksesta ja toiminnasta. **Metriikat** kuvaavat
määrällisiä arvoja, joiden avulla voi seurata sovelluksen toimintaa
tiivistetysti. Ne voivat esimerkiksi kertoa, kuinka monta asiakasta on
yhteydessä palvelimeen, kuinka monta viestiä palvelin käsittelee tietyn
ajanjakson aikana ja kuinka usein käsittely epäonnistuu.

Mittaustietojen kerääminen ajan mittaan auttaa havaitsemaan muutoksia palvelimen
toiminnassa. Esimerkiksi aktiivisten yhteyksien määrän kasvu yhdessä pitenevien
vasteaikojen kanssa voi viitata siihen, että palvelin lähestyy kapasiteettinsa
rajoja, ja jotain täytyisi tehdä sen skaalaamiseksi. Metriikat auttavat
tunnistamaan, milloin ongelma ilmenee, kun taas lokit ja jäljitykset auttavat
tutkimaan sen taustalla olevia yksittäisiä tapahtumia.

### Erilaisia metriikoita

Palvelimen toimintaa voi mitata eri tyyppisillä metriikoilla:

- **Laskuri (counter)** mittaa tapahtumien kokonaismäärää mittauksen alusta
  lähtien (yleensä palvelimen käynnistymisestä). Palvelin voi esimerkiksi
  laskea hyväksyttyjen yhteyksien, vastaanotettujen viestien tai käsittelyssä
  tapahtuneiden virheiden kokonaismäärää.

  Kun palvelin on käynnissä pitkään, jatkuvasti kasvava laskuri voi menettää
  hyödyllisyytensä. Siksi voi olla hyödyllistä jakaa laskuri tietyn pituisiin
  ajanjaksoihin, jotta trendimuutoksia on helpompi nähdä, esimerkiksi
  tunneittain tai päivittäin. Tällaiset mittarit voisi näinollen raportoida
  sarjana (tallentaen esimerkiksi Rust-vektoriin) yksittäisen arvon sijaan.

- **Mittari (gauge)** mittaa järjestelmän tämän hetkistä tilaa. Se voi
  kuvata esimerkiksi aktiivisten yhteyksien määrää, jonossa odottavien
  käsittelemättömien viestien määrää tai muistin käyttöä.

- **Histogrammi (histogram)** mittaa arvojen jakautumista määritellyllä
  vaihteluvälillä. Histogrammi voi esittää pyyntöjen käsittelyaikojen tai
  saapuvien ja lähtevien viestien kokojen jakauman. Se kertoo palvelimen
  toiminnan vaihtelusta paremmin kuin pelkkä keskiarvo: jos suuri osa
  yhteyksistä toimii hyvin ja pieni osa huonosti, histogrammi voi tuoda tämän
  esiin, kun taas keskiarvo saattaa piilottaa huonosti toimivan vähemmistön.

  Histogrammin keräämistä varten määritellään yleensä luokat eli korit
  (buckets), jotka vastaavat eri mittausvälejä, ja lasketaan kuhunkin koriin
  osuvien havaintojen määrä. Alla on esimerkki HTTP-pyyntöjen kestoja
  kuvaavasta histogrammista blogikirjoituksesta "[Prometheus Metrics Explained:
  Counters, Gauges, Histograms &
  Summaries](https://victoriametrics.com/blog/prometheus-monitoring-metrics-counters-gauges-histogram-summaries/)".

  ![HTTP-pyyntöjen kestojen histogrammi](https://victoriametrics.com/blog/prometheus-monitoring-metrics-counters-gauges-histogram-summaries/histogram-request-duration.webp){: width="90%" .center-img }

Rustissa käsittelyaikaa voi mitata tallentamalla aloitushetken kutsulla
`std::time::Instant::now()` ja kutsumalla sen jälkeen `.elapsed()`-metodia, kun
mitattava toiminto on valmis. Metodi palauttaa `Duration`-tyyppisen arvon, jonka
funktio `.as_secs_f64()` muuntaa sekunneiksi desimaaliosineen:

```rust
let started = std::time::Instant::now();
// Käsittele pyyntö tässä.
let processing_seconds = started.elapsed().as_secs_f64();
```

## Valvonta

Tuotannossa olevan palvelimen toiminnan analysointiin on tarjolla **valvonta- ja
visualisointityökaluja**, jotka seuraavat käynnissä olevan palvelimen tilaa ja
suorituskykyä edellä kuvattujen metriikkojen, lokien ja jäljitystietojen avulla.
Havaintojen jatkuva kerääminen auttaa tunnistamaan kuormituksen muutoksia ja
havaitsemaan ongelmia, joita ei välttämättä ilmene testauksen aikana. Alla
esitellään lyhyesti **Prometheus** ja **Grafana**, joihin kurssilla ei keskitytä
tämä syvällisemmin, mutta joita voit halutessasi kokeilla, jos aikaa ja
kiinnostusta riittää.

Jotta metriikat ovat käytettävissä sovelluksen ulkopuolella, palvelimen on
annettava ne ulkoisten työkalujen saataville julkisen rajapinnan kautta.
Esimerkiksi [Prometheus](https://prometheus.io/docs/introduction/overview/) on
yleisesti käytetty työkalu, joka kerää metriikkoja tietystä HTTP-päätepisteestä,
tyypillisesti polusta `/metrics`. Palvelimen tulee tarjota metriikat siellä
Prometheuksen ymmärtämässä muodossa. Prometheus tallentaa mittausarvot
aikaleimoineen, jolloin niiden historiaa voidaan tarkastella ja laskea
esimerkiksi tietyllä ajanjaksolla käsiteltyjen viestien määrä.

**Ohjauspaneeli** (dashboard) esittää mittaustulokset kuvaajina ja ajantasaisina
arvoina.
[Grafana](https://grafana.com/docs/grafana/latest/fundamentals/dashboards-overview/)
on toinen yleisesti käytetty työkalu, joka voi hakea tietoja esimerkiksi
Prometheuksesta ja näyttää toisiinsa liittyviä mittaustuloksia yhdessä.
Palvelimen kannalta ohjauspaneeli voisi näyttää esimerkiksi
aktiiviset yhteydet, viestien läpimenonopeuden, virheiden esiintymistaajuuden ja
pyyntöjen käsittelyajat tietyllä ajanhetkellä. Kuvaajien vertailu samalla
aikavälillä auttaa hahmottamaan, miten palvelimen kuormitus vaikuttaa
suorituskykyyn.

**Hälytykset** ilmoittavat palvelimen ylläpitäjälle huomiota vaativista
tilanteista, jotta ohjauspaneelia ei tarvitse seurata jatkuvasti. Hälytys voi
esimerkiksi kertoa, että palvelimeen ei saada yhteyttä, tai että virheiden
esiintymistiheys on ylittänyt määritellyn raja-arvon jonkun ajanjakosn aikana.
Hälytyksen tulisi kuvata ongelma riittävän tarkasti, joita ylläpitäjä voi
reagoida siihen vaadittavalla tavalla.

<div class="assignment-frame" markdown="1">

## Tehtävä #5

**Osa 1**: Jos et ole vielä tehnyt niin, lisää toteutukseesi asianmukainen
virheenkäsittely kohtiin, joissa suoritus voi joskus epäonnistua, erityisesti
operaatioihin jotka liittyvät tietoliikenteeseen. Käytä sopivissa kohdissa
`error!`- ja `warn!`-makroja näiden tapahtumien kirjaamiseen lokiin. Lisää myös
`info!`-tapahtuma, kun uusi yhteys hyväksytään.

**Osa 2**: Toteuta vähintään yksi yksikkötesti valitsemallesi toteutuksen
toiminnolle. Jos et ole vielä tehnyt niin, siirrä jokin sopiva toiminto
(esimerkiksi viestin otsakkeen jäsentäminen) erilliseen funktioon, jota voidaan
testata. Kuvaa raportissasi lyhyesti, mitä toimintoa testasit ja millaisia
tapauksia testin assert-operaatioilla tarkistettiin.

**Osa 3**: Toteuta päästä päähän -testejä varten testiasiakas, joka muodostaa
yhteyden palvelimeen ja testaa TST-viestiä, käyttäjän rekisteröintiä
USR-viestillä ja viestien lähettämistä MSG-viestillä erilaisilla kelvollisilla
ja mahdollisesti virheellisillä viestisarjoilla. Kokeile myös tuntemattoman
viestityypin lähettämistä. Testaa, että oma palvelimesi selviää testistä, ja tee
tarvittavat korjaukset, jos näin ei ole. Testaa sitten **kahta muuta palvelinta**,
jotka on listattu kurssin palvelimella ja jotka ilmoittavat tukevansa protokollan
versiota "base-2". Kuvaa raportissasi testitapauksesi ja testien havainnot.

Eri palvelinten testaamisen helpottamiseksi yhteyden kohdeosoite ja portti
kannattaa määrittää komentoriviargumenteilla. Voit sijoittaa testiasiakkaan
erilliseksi ohjelmaksi `bin`-hakemistoon esimerkiksi nimellä "_testclient.rs_".
Katso esimerkeistä kuinka komentoriviargumentteja käytetään, ja esimerkiksi
[projektipohjan
asiakas](https://github.com/PasiSa/pronets/blob/main/examples/project-template/client/src/main.rs)
näyttää kuinka **clap** - kirjaston parsintaa käytetään.

**Osa 4**: Kerää metriikoita vähintään avattujen yhteyksien, vastaanotettujen
viestien ja virhetapahtumien määristä. Toteuta **MET**-viesti, joka palauttaa
palvelimella kerättyjen metriikoiden nykytilan. Palvelimen tulee vastata
MET-viestillä, jolla on sama tunniste kuin pyynnöllä. Yhteisen otsakkeen jälkeen
metriikat tulee sisällyttää viestiin JSON-muotoisena merkkijonona. Kannattaa
muistaa edellisessä moduulissa mainittu **serde**-kirjasto, jolla
tietorakenteita voidaan sarjallistaa JSON-muotoon. Sisällytä vähintään seuraavat
metriikat:

- `"connections_total"`: Palvelimeen avattujen yhteyksien kokonaismäärä.
- `"messages_received"`: Vastaanotettujen viestien kokonaismäärä.
- `"errors"`: Palvelimen virhetapahtumien kokonaismäärä (virheistä pitäisi olla
  lisätietoja tapahtumalokissa).

Voit halutessasi kerätä myös muita metriikoita.

Jos olet päivittänyt protokollasuunnitelmaasi ja toteutustasi, päivitä lopuksi
myös `doc/design.md` tarpeen mukaan. Kun olet toteuttanut ja testannut yllä
olevat osat, tee muutoksistasi commit ja push tavalliseen tapaan ja päivitä
palvelininstanssi `/run-docker`-päätepisteen kautta (kannattaa myös tehdä
väli-committeja aiemmissa vaiheissa, esimerkiksi kunkin edellämainitun osion
jälkeen). Käytä protokollan tunnisteena "**base-3**" tai omaa
protokollatunnistettasi, jos olet aloittanut projektikohtaisen protokollasi
toteuttamisen. Tällöinkin sinun tulee toteuttaa myös "**base-3**"-määritykset.

Sisällytä raporttiisi myös seuraavat tiedot:

- Kuinka paljon aikaa käytit tehtävään?
- Mikä tehtävässä oli helppoa tai vaikeaa?
- Mitä työkaluja tai muita tietolähteitä käytit? Varsinkin jos käytit
  tekoälyavustimia, kerro miten käytit niitä ja olivatko ne hyödyllisiä.

</div>
