---
title: Protokollasuunnittelu ja projektit
lang: fi-FI
translation_key: protocol
---

Tässä kurssin osassa aloitetaan projekti, jonka parissa työskennellään kurssin
loppuun asti. Seuraavien viikkojen aikana projektia kehitetään kunkin viikon
teeman mukaisesti (testaus ja havainnointi, rinnakkaisuuden parantaminen,
tietoturva jne.). Tällä moduulilla ei ole erityisiä teknisiä oppimistavoitteita,
vaan sen tarkoituksena on antaa reunaehdot ja odotukset projektityölle.

## Yhteinen protokollamäärittely

Seuraavassa määritellään projekteissa käytettävälle tiedonsiirtoprotokollalle
joitakin perusominaisuuksia ja toimintoja. Näiden lisäksi voit suunnitella oman
protokollasi ja sen toteutuksen valitsemasi aiheen mukaisesti. Jokaisella
protokollalla on tunniste muodossa "`nimi-NN`", jossa "_nimi_" on protokollan
nimi ja "_NN_" on protokollan versiota ilmaiseva positiivinen kokonaisluku.
Protokollaversioiden tulee olla taaksepäin yhteensopivia: version "_nimi-2_"
tulee tukea kaikkia version "_nimi-1_" viestejä, mutta se sisältää myös uusia
ominaisuuksia. Tämä tunniste sisällytetään edellisessä moduulissa esitellyn
`/run-docker`-pyynnön _protocol_-kenttään, jolloin rakennetaan uusi versio
protokollan toteuttavasta palvelinkontista ja käynnistetään se kurssin
palvelimella.

Kaikkien projektien tulee toteuttaa yhteinen perusprotokolla. Sillä on yhteinen
viestikehyksen rakenne, jota toteutusten tulee noudattaa. Perusprotokollan
toiminnot on jaettu kolmeen versioon edellä kuvatun nimeämiskäytännön
mukaisesti:

- `base-1`: sisältää **TST**-viestin, jolla testataan asiakkaan ja palvelimen
  välistä yhteyttä. Tämä toteutettiin jo tehtävässä 3.
- `base-2` (toteutetaan tehtävässä 4, joka kuvataan tämän sivun lopussa):
  lisää **USR**-viestin, jolla rekisteröidään asiakasohjelmaa käyttävä nimetty
  käyttäjä, sekä **MSG**-viestin, jolla lähetetään viesti muille palvelimeen
  yhdistetyille käyttäjille.
- `base-3` (toteutetaan tehtävässä 5): lisää **MET**-viestin, jolla kysytään
  palvelimelta käyttö- ja suorituskykymittareita.

Kaikkien laajennettujen protokollatoteutusten tulee toteuttaa "base-3"-version
ominaisuudet ja viestirakenne itse suunniteltujen laajennusten lisäksi.

### Viestikehyksen rakenne

Koska viestinnässä käytetään tietovirtapohjaista TCP-protokollaa, joka ei
säilytä viestien rajoja, tarvitaan tapa erottaa viestit toisistaan
sovelluskerroksella. Yksi yleinen tapa on määritellä viesteille yhteinen otsake,
joka kertoo viestin hyötykuorman pituuden esimerkiksi tavuina. Toinen tapa, jota
käytetään muun muassa HTTP:n vanhoissa versioissa ja muissa perinteisissä
tekstipohjaisissa protokollissa, on määritellä viestin lopun ilmaiseva
erikoismerkki tai merkkijono (esimerkiksi kaksi peräkkäistä rivinvaihtomerkkiä).
Yhteisessä protokollamäärittelyssämme käytämme ensimmäistä tapaa.

Kaikkien protokollaviestien tulee alkaa seuraavalla otsakkeella:

- Viesti alkaa **viestin pituudella** tavuina: tämä on 32-bittinen etumerkitön
  kokonaisluku big-endian tavujärjestyksessä. Pituus ilmaisee koko viestin pituuden
  tavuina (oktetteina), **pituus- ja tunnistekentät mukaan lukien**.
- Toinen kenttä on **viestin tunniste**: tämäkin on 32-bittinen etumerkitön
  kokonaisluku verkkotavujärjestyksessä. Tunnisteen tarkoituksena on yhdistää
  pyyntöviestit niiden vastauksiin. Helpoin toteutustapa on käyttää kasvavia
  etumerkittömiä kokonaislukuja kuhunkin viestiin, mutta muitakin jakotapoja voi
  käyttää, kunhan samaa tunnistetta ei käytetä kahdesti saman yhteyden aikana.
  Poikkeuksena on tilanne, jossa kaikki 32-bittiset tunnisteet on jo käytetty:
  silloin tunnisteiden jakamisen voi aloittaa alusta.
- Kolme merkkiä ja niitä seuraava välilyönti ilmaisevat **viestin tyypin**.
  Tunnisteen tulisi käyttää isoja kirjaimia A–Z, sekä tarvittaessa numeromerkkiä.
- Tämän jälkeen tuleva sisältö riippuu viestistä, kuten alla kuvataan.

Alla vielä uudestaan edellisessä moduulissa esitetty kuva, joka havainnollistaa
yllä olevaa rakennetta:

![Viestiformaatti](/images/protocol-format.svg){: width="90%" .center-img }

### Yhteiset viestit, "base-1"

- **TST** (testi). Asiakas tai palvelin voi lähettää tämän viestin testatakseen,
  että yhteys toimii ja toinen osapuoli vastaa, tai esimerkiksi mitatakseen
  yhteyden viivettä.
  - **Parametrit:**
    - **Testiviesti**: Tavujono, joka toisen osapuolen tulee kaiuttaa takaisin.
      Tavujen muoto on vapaa: ne voivat muodostaa UTF-8-koodatun merkkijonon tai
      minkä tahansa binäärisen tavujonon. Testijono voi olla pitkä (jopa lähes 4
      Gt), sillä sen pituutta rajoittaa vain viestin kokonaispituus.
  - **Vastaus:** Sama viesti kaiutettuna takaisin toiselle osapuolelle.
    Siksi myös viestin pituus ja tunniste ovat samat kuin pyyntöviestissä.
  - **Esimerkki:** [pituus: 22]`TST 0123456789`

### Yhteiset viestit, "base-2"

TST-viestin lisäksi:

- **USR** (käyttäjän rekisteröinti). Asiakas lähettää tämän viestin liittääkseen
  käyttäjänimen nykyiseen TCP-yhteyteen. Tämä tulee lähettää ensimmäisenä
  viestinä yhteyden muodostamisen jälkeen (poikkeuksena sallitaan TST-viesti,
  joka hyväksytään aina).
  - **Parametrit:**
    - **Käyttäjänimi**: UTF-8-koodattu merkkijono USR-viestityypin jälkeen.
      Merkkijonon pituus voidaan päätellä viestin alussa olevasta
      pituuskentästä.
  - **Vastaus:** Palvelin vastaa kaiuttamalla saman viestin takaisin tai
    lähettämällä **ERR**-viestin, jos rekisteröinti epäonnistui esimerkiksi
    siksi, että nimi oli jo varattu. Vastauksella tulee olla sama tunniste
    kuin pyyntöviestillä.
  - **Esimerkki:** [pituus: 18]`USR Jaakko` -- Rekisteröi käyttäjän "Jaakko"
    tälle TCP-yhteydelle.

- **MSG** (viesti asiakkaalta). Lähettää viestin annetulle keskustelukanavalle.
  Viesti välitetään kaikille kyseisen kanavan jäsenille. Huomaa, että
  palvelimen täytyy varata vapaa viestitunniste erikseen kullekin
  vastaanottajalle, koska tunnisteiden numerointi on yhteyskohtainen.
  Lähettäjälle annettavassa vastauksessa käytetään samaa tunnistetta kuin
  asiakkaan lähettämässä pyynnössä.
  - **Parametrit:**
    - **Kanava**: Sen kanavan nimi, jolle keskusteluviesti lähetetään. Viesti
      tulee toimittaa kaikille kanavan jäseniksi rekisteröityneille käyttäjille.
      Toteutuksen on aina sisällettävä "**general**" - kanavaa, johon kuuluvat
      kaikki palvelimeen yhdistetyt käyttäjät. Muiden kanavien tukeminen on
      vapaaehtoista.
    - Välilyönti
    - **Viesti**: Viesti UTF-8-koodattuna tekstinä (Rust-merkkijonojen
      oletuskoodaus). Viesti voi sisältää mitä tahansa UTF-8-merkkejä,
      esimerkiksi rivinvaihtoja. Viestin pituus määritetään protokollaviestin
      alussa olevasta pituuskentästä huomioiden myös protokollaviestin muut
      kentät.
  - **Vastaus:** Ei varsinaista vastausta, mutta myös lähettäjä saa palvelimen
    välittämän MSG-viestin, jolla palvelin varaa erillisen viestitunnisteen.
    Virhetilanteessa (esimerkiksi virheellinen kanavan nimi) palvelimen tulee
    vastata **ERR**-viestillä, jossa on sama tunniste kuin pyynnössä sekä
    virheen syy.
  - **Esimerkki:** [pituus: 45]`MSG general Hello, how are you doing?`

- **MSG** (viesti palvelimelta). Palvelin käyttää tätä viestiä välittääkseen
  käyttäjän viestin kaikille kanavan jäsenille. Palvelimen lähettämän viestin
  hyötykuorma eroaa hieman asiakkaan lähettämän viestin hyötykuormasta.
  Huomaa, että palvelin varaa viestitunnisteen: se ei ole sama kuin asiakkaan
  lähettämässä MSG-viestissä. Palvelimen ei tule käyttää samaa viestitunnistetta
  kahdesti saman asiakkaan kanssa.
  - **Parametrit:**
    - **Lähettäjä**: Viestin lähettäjän käyttäjänimi.
    - Välilyönti
    - **Kanava**: Sen kanavan nimi, jolle viesti on tarkoitettu.
    - Välilyönti
    - **Viesti**: Viesti UTF-8-koodattuna tekstinä.
  - **Vastaus:** Ei vastausta.
  - **Esimerkki:** [pituus: 52]`MSG Jaakko general Hello, how are you doing?`

- **ERR** (virheviesti palvelimelta). Lähetetään vastauksena pyyntöviestiin,
  jos pyynnön suorittaminen epäonnistuu. Viestin tunnisteen tulee olla sama
  kuin virheen aiheuttaneessa pyyntöviestissä.
  - **Parametrit:**
    - **Syy**: Virheen syy UTF-8-koodattuna tekstinä.
  - **Vastaus:** Ei vastausta.
  - **Esimerkki:** [pituus: 35]`ERR Username already taken!`

### Yhteiset viestit, "base-3"

_TODO: protokollan yksityiskohdat lisätään myöhemmin_

## Protokollasuunnittelun periaatteet

Internet-protokollien kehitys alkoi 1970-luvulla. Vaikka protokollat ovat
kehittyneet vuosikymmenten aikana ja uusia sovelluksia ja teknologioita on
syntynyt, varhaisissa protokollissa valitut suunnitteluperiaatteet ovat
osoittautuneet kestäviksi, ja vanhoja protokollateknologiota on monesti pystytty
hyödyntämään uusilla ja kekseliäillä tavoilla.

Internet-protokollia standardoivaan **[IETF:n](https://www.ietf.org/)** -
järjestöön kuuluva **[Internet Architecture Board](https://www.iab.org/about/)**
julkaisi vuonna 1996 RFC-dokumentin **[RFC 1958, "Architectural Principles of
the Internet"](https://datatracker.ietf.org/doc/html/rfc1958)**. Siinä
ehdotetaan uusien suunnitteluperiaatteita Internet-protokollien kehittämiseen,
jotka ryhmä kokeneita Internet-kehittäjiä on koonnut. Vaikka RFC-dokumentti on
yli 30 vuotta vanha, periaatteet kannattaa edelleen huomioida protokollia
suunnitellessa. Monet niistä ovat hyödyllisiä myös yleisinä ohjelmistojen
suunnitteluperiaatteina.

Alla ovat RFC-dokumentin "General Design Issues" -osion periaatteet höystettynä
allekirjoittaneen omilla lisäkommenteilla (joista saa olla argumentoida):

1. **Heterogeenisuus on väistämätöntä, ja se on huomioitava suunnittelussa.**
   Lähihistoria on osoittanut, että tiedonsiirtoteknologiat ja käyttötavat ovat
   muuttuneet merkittävästi. On huomioitava hyvin erilaisia toimintaympäristöjä:
   langattomien laitteiden Internet-yhteydet voivat olla epäluotettavia, ja
   tiedonsiirtonopeudet voivat vaihdella huomattavasti. Joskus tiedonsiirto
   tapahtuu satelliittiyhteyden kautta, mikä aiheuttaa pitkiä viiveitä. Nämä
   seikat on hyvä pitää mielessä, kun suunnitellaan sovelluksen ajoituksista
   riippuvaa toimintalogiikkaa tai sen kykyä selviytyä häiriötilanteista.

2. **Jos saman asian voi tehdä usealla tavalla, valitse yksi.** Eri
   verkkosovelluksissa ja protokollissa joudutaan usein ratkaisemaan
   samankaltaisia robustisuuteen ja luotettavuuteen liittyviä haasteita.
   Sen sijaan että lähdetään kehittämään uutta hienoa lähestymistapaa voi olla
   hyvä tarkastella, miten samanlainen ongelma on ratkaistu aiemmin ja voisiko
   samoja ratkaisuja käyttää uudelleen.

3. **Kaikkien ratkaisujen on skaalauduttava helposti erittäin suureen määrään
   solmuja kussakin verkkoympäristössä, sekä useiden miljoonien verkkojen
   välillä.** Tällä kurssilla ei testata skaalautuvuutta miljooniin verkkoihin.
   Skaalautuvuus on silti tärkeä tavoite sekä protokollasuunnittelussa että
   verkkopalvelimen toteutuksessa, esimerkiksi suunniteltaessa tietorakenteita
   tai viestien käsittelyyn käytettäviä algoritmeja.

4. **Toiminnallisuuden lisäksi on huomioitava suorituskyky ja kustannukset.**
   Uutta sovellusta kehitettäessä huomio kiinnittyy luonnollisesti sen
   toiminnallisuuteen ja ominaisuuksiin. Samalla tulisi tarkastella myös
   suorituskykyä: esimerkiksi tiedonsiirtoprotokollan aiheuttamaa ylimääräistä
   kuormitusta ja viiveitä, prosessoinnin tehokkuutta sekä muistinkäytön
   taloudellisuutta.

5. **Pidä ratkaisu yksinkertaisena. Jos olet suunnitteluvaiheessa epävarma,
   valitse yksinkertaisin ratkaisu.** Tämä on hyvä periaate kaikissa tilanteissa:
   protokollasuunnittelun ja ohjelmistokehityksen lisäksi myös elämässä yleensä.
   Erityisesti aloittelijat tekevät usein sen virheen, että suunnittelevat
   liian monimutkaisen ratkaisun eivätkä huomaa yksinkertaisempaa ja helpompaa
   lähestymistapaa.

6. **Modulaarisuus on hyvä asia. Jos voit pitää asiat erillään, tee niin.** Tämä
   on yleisesti hyvä periaate ohjelmistokehityksessä. Protokollasuunnittelussa
   perinteinen protokollien kerrosrakenne on yksi tapa soveltaa tätä
   periaatetta. Kannattaa hyödyntää yhteisiä protokollarakenteita ja
   uusiokäyttää esimerkiksi viestien parsimiseen liittyviä funktioita, kuten
   meidän tapauksessamme edellä kuvatun yhteisen kehysotsakkeen käsittelyssä.

7. **Monissa tapauksissa on parempi ottaa lähes valmis ratkaisu käyttöön nyt
   kuin odottaa täydellisen ratkaisun löytymistä.** Elinaikamme on rajallinen,
   ja niin on myös tämän kurssin kesto.

8. **Vältä valinnaisia asetuksia ja parametreja aina kun mahdollista.
   Tarvittavat asetukset ja parametrit tulisi määrittää tai neuvotella
   dynaamisesti eikä asettaa käsin.** Tämä liittyy ensimmäiseen periaatteeseen:
   koska verkkoympäristöt ja teknologiat eroavat toisistaan huomattavasti,
   esimerkiksi ajastimien pituuksien valitseminen käsin voi osoittautua
   lyhytikäiseksi ratkaisuksi. Vaikka jotkin arvot toimivat nykyisessä
   verkkoympäristössä, ne eivät välttämättä toimi enää muutaman vuoden kuluttua.
   Yksittäisiä toteutuksia voidaan päivittää ohjelmistopäivityksillä, mutta
   samoista protokollista on useita toteutuksia eri järjestelmissä. Siksi
   protokollien muuttaminen kattavasti käyttöönoton jälkeen on vaikeampaa.

9. **Ole tarkka lähettäessäsi ja salliva vastaanottaessasi.** Tämä on yleinen
   robustisuutta edistävä periaate protokollatoteutuksissa. Kun viestejä
   lähetetään verkkoon, protokollamäärittelyä ja sen sääntöjä tulee noudattaa
   huolellisesti, jotta vastaanottajalle ei aiheudu ongelmia. Viestejä
   vastaanotettaessa tulee pyrkiä käsittelemään hallitusti kaikenlaiset
   tiedonsiirron virhetilanteet (virheelliset tai epäloogiset arvot,
   tunnistamattomat protokollamuodot, jne.). Osa virheistä on tahattomia:
   esimerkiksi toisen toteutuksen tekijä on saattanut ymmärtää
   protokollamäärittelyn väärin. Osa voi olla tahallisia, esimerkiksi
   hyökkääjän yrityksiä löytää haavoittuvuuksia.

10. **Lähetä pyytämättä lähetettäviä paketteja säästeliäästi,
    erityisesti ryhmä- ja yleislähetyksiä.** Protokollaliikenteessä kannattaa
    aina pyrkiä taloudellisuuteen. Verkkoviestintä kuluttaa energiaa eri
    mittakaavoissa: verkon reitittimet ja datakeskukset kuluttavat paljon
    energiaa, mutta kulutusta syntyy myös pienemmässä mittakaavassa, esimerkiksi
    jos langaton akkukäyttöinen laite täytyy herättää sellaisen viestin vuoksi,
    jonka lähettäminen olisi voitu välttää.

11. **Kehäriippuvuuksia on vältettävä.** Tämä on hyvä sääntö kaikessa
    ohjelmistosuunnittelussa.

12. **Siirrettävien rakenteiden tulisi olla kohtuullisissa rajoissa itseään
    kuvaavia (sisältää tyyppi ja koko).** Protokollasta voi olla erilaisia
    kehitysmuotoja ja versioita, eivätkä kaikki toteutukset välttämättä tue
    kaikkia ominaisuuksia. Siksi protokolla tulisi suunnitella niin, että
    tuntemattomat viestit voidaan helposti ohittaa ja esimerkiksi
    TCP-tietovirrasta löydetään kohta, josta käsittelyä jatketaan.

13. **Kaikissa määrittelyissä tulisi käyttää samaa terminologiaa ja
    merkintätapaa sekä samaa bitti- ja tavujärjestystä.** Tämä on ensisijaisesti
    IETF:n standardointityötä koskeva ohje, mutta hyvä periaate myös kurssin
    toteutuksissa. Käytämme verkkotavujärjestystä (big-endian), kuten IETF:n
    määrittelemät protokollat tekevät.

14. **Mitään ei standardoida ennen kuin siitä on useita toimivia toteutuksia.**
    Tämä on IETF:n standardointia koskeva ohje, mutta myös tällä kurssilla
    tavoitteena on päätyä muutamaan yhteiseen protokollamäärittelyyn, joista
    kustakin on useampi kuin yksi riippumaton asiakas- ja palvelintoteutus.

## Perusprotokollasta verkkosovellukseksi

Tässä vaiheessa voit valita projektiaiheen, jonka parissa työskentelet kurssin
loppuun asti. Kaikille projekteille on seuraavat yhteiset vaatimukset:

- Projektien tulee toteuttaa perusprotokolla kurssimateriaalin määrittelyn
  mukaisesti. Uusien projektikohtaisten protokollaviestien tulee käyttää
  TCP-yhteydellä samaa otsakerakennetta (pituus, tunniste, tyypin nimi) kuin
  perusprotokollan viesteissä.
- Palvelintoteutusten tulee pystyä palvelemaan useita asiakkaita samanaikaisesti
  ja riittävän nopeasti.
- Ennen kurssin loppua tiedonsiirrossa tulee käyttää TLS-suojausta
  (lukuun ottamatta kurssin loppupuolella toteutettavaa UDP-tiedonsiirtoa).
- Projektiin tulee sisältyä reaaliaikaista tiedonsiirtoa UDP:n avulla. Tämä voi
  liittyä esimerkiksi interaktioon yhteisen dokumentin kanssa, jokin moninpelin
  reaaliaikainen osa tai haastavampana vaihtoehtona äänikeskustelu yhdistettyjen
  asiakkaiden välillä (Rustille on saatavilla kirjastoja, joilla mikrofonilla
  kerättyä ääntä voidaan tallentaa datalohkoiksi valitulla koodauksella ja
  toistaa niitä).

Tavoitteena on, että kustakin projektiaiheesta syntyy vähintään kaksi
riippumatonta asiakas- ja palvelintoteutusta kahdelta eri ryhmältä (tai
henkilöltä, jos työskentelet yksin). Eri toteutuksissa voi olla erilaisia
ominaisuuksia, mutta tavoitteena on, että kunkin aiheen toteutuksilla on myös
yhteisiä protokollaviestejä, joita ne ymmärtävät keskenään.

## Projektiaiheet

Voit valita projektin seuraavista aiheista. Kaikkiin projekteihin sisältyy
keskustelutoiminto, joka toteutettiin aiemmassa tehtävässä osana
perusprotokollaa. Seuraavissa kuvauksissa ehdotetaan mahdollisia ominaisuuksia
ja aiheisiin mahdollisesti liittyviä tiedonsiirtotarpeita, mutta varsinaiset
protokollaviestit määritellään yhdessä muiden saman aiheen parissa
työskentelevien kanssa.

### Yhteiskäyttöinen valkotaulu

Käyttäjät voivat liittyä kaksiulotteiselle valkotaululle, jolle he
piirtävät erilaisia elementtejä. Tauluun tulisi voida lisätä ainakin tekstiä,
suorakulmioita ja viivoja, mutta voit toteuttaa myös lisäominaisuuksia, kuten
muita muotoja, värien valitsemisen ja objektien siirtämisen. Halutessasi voit
tukea useita nimettyjä valkotauluja samaan tapaan kuin keskusteluhuoneita voi
olla useita.

Todennäköisesti tarvittavat protokollatoiminnot:

- Elementin lisääminen annettuihin koordinaatteihin. Viestin tulee sisältää myös
  elementin tyypistä riippuvat ominaisuudet (esimerkiksi lisättävä teksti,
  suorakulmion mitat, jne.). Kun yksi asiakas lisää elementin, myös muiden
  valkotauluun liittyneiden käyttäjien tulee nähdä se.

- Valkotaulun kaikkien nykyisten elementtien hakeminen. Kun uusi asiakas liittyy
  olemassa olevalle valkotaululle, sen täytyy pyytää taulun nykyinen tila.
  Vastauksena tähän viestiin voisi lähettää esimerkiksi sarjan edellä kuvattuja
  lisäysviestejä aiemmin lisätyistä elementeistä.

- Valkotaulun tyhjentäminen. Koska palvelinistunnot ovat pitkäkestoisia,
  valkotaulu voi ajan mittaan muuttua sekavaksi. Siksi sen palauttaminen
  tyhjään tilaan voi olla hyödyllistä.

Ehdotus reaaliaikaisesta toiminnosta (joka toteutetaan UDP:llä):

- Vuorovaikutteinen osoitin, jossa eri asiakkailla voisi olla käytössä
  esimerkiksi eri värit.

### Yhteiskäyttöinen dokumenttieditori

Asiakkaat työstävät yhteistä tekstidokumenttia, jota voi muokata
samanaikaisesti. Tässä aiheessa kannattaa kiinnittää huomiota siihen, miten
samaan kohtaan kohdistuvat muokkaukset käsitellään ja miten dokumentti pidetään
yhtenäisenä eri asiakkailla. Halutessasi voit toteuttaa erilaisia tapoja
korostaa dokumentin sisältöä (värejä, tehosteita tai muotoja). Yhtenä
mahdollisena lisäominaisuutena dokumentin voisi myös ladata omalle koneelle tai
aiemmin laaditun tekstin voisi siirtää muokkauksen pohjaksi.

Todennäköisesti tarvittavat protokollatoiminnot:

- Tekstin lisääminen annetulle riville ja sarakkeeseen. Tekstin lisääminen
  merkki kerrallaan voi olla tiedonsiirron kannalta tehotonta, joten kannattaa
  harkita tekstin lisäämistä suurempina kokonaisuuksina.

- Dokumentin nykyisen tilan hakeminen. Myöhemmin liittyvät asiakkaat tarvitsevat
  tätä. Toimintoa voi käyttää myös varmistamaan, että dokumentin sisältö on
  sama kaikilla asiakkailla.

- Sisällön poistaminen dokumentista.

Ehdotus reaaliaikaisesta toiminnosta (joka toteutetaan UDP:llä):

- Vuorovaikutteinen kohdistin, josta näkee, missä käyttäjä parhaillaan
  työskentelee.

### Monen pelaajan autopeli

Yksinkertainen kaksiulotteinen peli, jossa useat pelaajat ajavat kilpaa radalla
tai vapaassa ympäristössä, jossa on esteitä. Hieno grafiikka ei ole tämän
projektin suunnittelutavoite, mutta verkosta löytyy helposti vapaasti
käytettävissä olevaa sprite-grafiikkaa (esim. autoja varten). Palvelimen ja
asiakkaiden tulee seurata kunkin auton sijaintia, suuntaa ja nopeutta. Autoille
voi toteuttaa myös muita ominaisuuksia. Yksinkertaisimmassa toteutuksessa autot
voivat ajaa vapaasti, mutta peliin voi lisätä myös kilpailuelementin
(esimerkiksi kolmen kierroksen kilpailun ja ajanoton). Pelissä tulee olla myös
jonkinlainen törmäysten tunnistus, vaikka sen ei tarvitse olla tarkka.
Halutessasi voit lisätä itsenäisesti toimivia NPC-autoja, joita ohjaa erillinen
asiakasohjelma. Osa asiakasohjelmista olisi tällöin siis vuorovaikutuksessa
käyttäjän kanssa ja osa ainoastaan ohjaisi autoja. Jälkimmäiset eivät
välttämättä tarvitse lainkaan käyttöliittymää, vaan ne voivat toimia taustalla.

Koska verkkoviestinnässä on viivettä, palvelimen tulee ylläpitää kunkin auton
ensisijaista tilatietoa sijainnin, nopeuden ja kuluneen ajan perusteella.
Pelikokemuksen sujuvoittamiseksi myös asiakkaat voivat päivittää sijainteja
paikallisesti näiden tietojen avulla, mutta niiden tulee synkronoida
varsinainen tila palvelimelta riittävän usein.

Todennäköisesti tarvittavat protokollatoiminnot:

- Kilpailun alustaminen, mukaan lukien radan tiedot sekä mahdolliset esteet ja
  muut objektit.

- Auton lisääminen annettuun sijaintiin. Lisäksi tarvitaan pelaajan tunniste
  sekä ominaisuuksia, kuten väri, joiden avulla auton erottaa muista.

- Auton suunnan, sijainnin ja nopeuden päivittäminen.

- Auton poistuminen. Kun palvelin havaitsee yhteyden sulkeutuneen, sen tulee
  ilmoittaa siitä muille asiakkaille. Auto voidaan poistaa kilpailusta tai
  jättää hylättynä kentälle. Jälkimmäinen vaihtoehto mahdollistaisi pelaajan
  palaamisen peliin esimerkiksi lyhyen yhteyskatkon jälkeen.

- Pelin päättäminen. Jossain vaiheessa pelin tila täytyy tyhjentää uutta peliä
  varten ja ilmoittaa tästä asiakkaille.

Ehdotus reaaliaikaisesta toiminnosta (joka toteutetaan UDP:llä):

- Autojen sijainteja voidaan päivittää reaaliaikaisesti UDP:n avulla.

### Droonisimulaattori

Yhteisellä kaksiulotteisella kartalla on useita drooneja ja esteitä. Droonien
käyttötarkoituksen voi määritellä vapaasti: ne voivat esimerkiksi kuljettaa
lähetyksiä tai suorittaa valvontatehtäviä. Droonit voivat olla käyttäjän
ohjaamia tai itsenäisesti toimivia. Itsenäisiä drooneja ohjaava asiakasohjelma
ei välttämättä tarvitse käyttöliittymää. Sen sijaan voit toteuttaa erillisen
asiakasohjelman, jonka tehtävä on tarjota käyttöliittymä droonien toiminnan
seuraamiseen. Yksi lisäominaisuusvaihtoehto voisi olla, että yksi asiakkaista
toimii koordinaattorina, joka välittää tehtäviä vapaille drooneille, käyttäen
suunniteltua protokollaa.

Toteutuksessa on ehkä jonkun verran yhteistä autopelin kanssa, ja
protokollaviestit voisi suunnitella yhteensopiviksi esimerkiksi
sijaintipäivitysten sekä droonien simulaatioympäristöön liittymisen ja siitä
poistumisen osalta.

Todennäköisesti tarvittavat protokollatoiminnot:

- Simulaatioympäristön alustaminen.

- Droonin lisääminen annettuun sijaintiin.

- Droonin nopeuden päivittäminen (skenaariosta riippuen drooneilla voi olla
  myös suunta, jota seurataan).

- Droonin ohjaimen poistuminen. Voit päättää, katoaako drooni vai jääkö se
  kartalle mahdollista yhteyden muodostamista uudelleen varten.

- Simulaation päättäminen.

Ehdotus reaaliaikaisesta toiminnosta (joka toteutetaan UDP:llä):

- Sijainteja voidaan päivittää UDP:n avulla.

## Projektin tavoitteet

Kurssin lopussa hyvin onnistuneeseen projektiin pätevät seuraavat ominaisuudet:

- Projektista on dokumentti, joka kuvaa sen rajauksen, toteutetut ominaisuudet,
  käytetyt protokollaviestit sekä pääpiirteittäin sen, miten projektia on
  testattu. Erityisesti ohjelmiston kääntämiseen ja käyttöön tulee olla selkeät
  ohjeet.

- Muiden asiakkaiden ja palvelimien kanssa tehdyt yhteentoimivuustestit on
  dokumentoitu.

- Git-repositoriossa oleva koodi on hyvin jäsenneltyä ja loogisesti seurattavaa.

- Projektin toteutetut ominaisuudet toimivat ilman merkittäviä virheitä.

- Projektissa on kattavat testit, jotka keskittyvät erityisesti
  tiedonsiirtologiikkaan.

- Projekti toimii luotettavasti usean (vähintään neljän) samanaikaisen asiakkaan
  kanssa ja reagoi toimintoihin riittävän nopeasti. Samanaikaisten käyttäjien
  aiheuttamaa kuormitusta on testattu, ja testit on dokumentoitu.

- Projektin TCP-yhteyksissä käytetään TLS-suojattua tiedonsiirtoa.

- Projektissa on reaaliaikainen osa, joka käyttää UDP:tä.

Tiedonsiirtoon liittyvät lisäominaisuudet arvioidaan plussana, esimerkiksi niitä
joita aihekuvausten yhteydessä ehdotettiin, mutta niitä ei vaadita.

## Hyödyllisiä kirjastoja

Asiakasohjelman ulkoasu ei ole arviointiperuste, mutta jonkinlainen graafinen
käyttöliittymä tarvitaan, jotta asiakasohjelmalla voi tehdä mielekkäitä asioita.
Alla on joitakin kirjastoja, joita voit käyttää. Kaikkien mainittujen
pitäisi toimia eri järjestelmissä (Windows, Mac, Linux).

### Graafinen käyttöliittymä

- **[Slint](https://crates.io/crates/slint)** ([verkkosivu](https://slint.dev/))
  on työkalu graafisten käyttöliittymien rakentamiseen. Se käyttää
  deklaratiivista käyttöliittymäkieltä asettelujen, komponenttien, animaatioiden
  ja sovelluksen tilan kuvaamiseen ja pitää sovelluslogiikan erillään
  käyttöliittymästä. Slint tarjoaa muun muassa käyttöliittymäelementtejä, kuvia,
  muunnoksia, animaatioita ja tapahtumankäsittelyä. Se sopii erityisesti
  valkotaulun asiakasohjelmaan, mutta tukee myös animoituja graafisia objekteja,
  joten sitä voi käyttää myös yksinkertaiseen peliin tai droonisimulaatioon.

- **[egui](https://crates.io/crates/egui)** on graafinen käyttöliittymäkirjasto.
  Siinä sovellus kuvaa käyttöliittymän jokaisella piirrettävällä ruudulla sen
  sijaan, että ylläpitäisi pysyvää käyttöliittymäelementtien hierarkiaa. egui on
  suunniteltu helposti liitettäväksi Rust-sovelluksiin, ja se sopii hyvin
  työkaluihin, editoreihin, visualisointeihin, virheenjäljityskäyttöliittymiin
  ja vuorovaikutteisiin sovelluksiin. Egui tukee tavallisia
  käyttöliittymäelementtejä, vapaata piirtämistä, syötteiden käsittelyä,
  asetteluja ja grafiikkaa. Sitä voi käyttää sekä natiivina
  työpöytäkäyttöliittymänä että verkkoselaimessa WebAssemblyn avulla.

- **[Macroquad](https://crates.io/crates/macroquad)**
  ([verkkosivu](https://macroquad.rs/)) on pelinkehitys- ja grafiikkakirjasto,
  joka on suunniteltu kaksiulotteisten pelien, simulaatioiden ja muiden
  vuorovaikutteisten graafisten sovellusten luomiseen. Se tarjoaa yksinkertaisen
  ohjelmointirajapinnan ikkunoiden avaamiseen, muotojen ja tekstuurien
  piirtämiseen, näppäimistön ja hiiren syötteiden käsittelyyn, äänen
  toistamiseen sekä ruutukohtaisen pelisilmukan toteuttamiseen. Lisäksi se tukee
  esimerkiksi sprite-kuvien pyörittämistä, törmäysten tunnistamista
  sovelluslogiikan avulla ja tekstin piirtämistä. Macroquad on hyvä vaihtoehto
  erityisesti autopeliin tai drooniprojektiin, mutta valkotauluun se ei
  välttämättä sovellu yhtä hyvin. Macroquad-sovellukset voivat toimia natiivisti
  työpöydällä sekä verkkoselaimissa WebAssemblyn avulla.

  Verkossa julkaistu kirja "[Game development in Rust with
  Macroquad](https://mq.agical.se/)", on hyödyllinen esittely Macroquadin eri
  ominaisuuksista. Esimerkkikansiossamme on myös yksinkertainen
  [Macroquad-esimerkki]() jossa esitellään objektin pyöritystä ja liikuttamista
  näppäinkomentojen pohjalta.

- **[Bevy](https://crates.io/crates/bevy)** ([verkkosivu](https://bevy.org/)) on
  pelimoottori kaksi- ja kolmiulotteisten pelien sekä vuorovaikutteisten
  sovellusten rakentamiseen. Sen arkkitehtuuri perustuu Entity Component System
  (ECS) -malliin, jossa peliobjektit koostuvat tietoa sisältävistä
  komponenteista ja pelilogiikka toteutetaan näitä komponentteja käsittelevinä
  järjestelminä. Bevy tarjoaa integroidun tuen piirtämiselle, syötteille,
  äänelle, animaatioille, sekä lisäosien kautta fysiikalle. Monipuolisuudestaan
  huolimatta Bevy voi olla aloittelijalle vaikeampi omaksua kuin Macroquad.

### Muita työkaluja

- **[Clap](https://crates.io/crates/clap)** (lyhenne sanoista "Command Line
  Argument Parser") on hyödyllinen ja kätevä kirjasto komentoriviargumenttien
  parsimiseen tietorakenteeksi, jonka eri kenttiä on helppo lukea ja käyttää.

- **[Serde](https://crates.io/crates/serde)** on ohjelmistokehys
  Rust-tietorakenteiden sarjallistamiseen ja sarjallistuksen purkamiseen eri
  esitysmuodoissa, kuten JSON. Tätä voi hyödyntää sovelluksen tilan
  tallentamiseen levylle tai rakenteisen tiedon siirtämiseen protokollaviestien
  sisällä.

<div class="assignment-frame" markdown="1">

## Tehtävä #4

Tässä tehtävässä aloitetaan projektityö ja toteutetaan yhteisen perusprotokollan
"**base-2**"-osa. Lisäksi tarkistetaan, toimiiko **TST**-viesti kurssin
palvelimella saatavilla olevien muiden toteutusten kanssa.

**Osa 1:** Luo git-repositoriosi juureen kansio "_doc_" ja lisää sinne dokumentti
"**design.md**", jossa kuvaat lyhyesti projektisi ja toteuttamasi protokollan
**Markdown-muodossa** ([tässä yksi
opas](https://www.markdownguide.org/basic-syntax/)). Git-repositorioon ei kannata
lisätä tarpeettomia binääritiedostoja, kuten PDF-dokumentteja, koska ne
kasvattavat sen kokoa nopeasti. Siksi käytämme tässä Markdownia.

Tämä on dokumentin ensimmäinen versio, ja sitä voi päivittää tulevien viikkojen
aikana suunnitelmien tarkentuessa tai muuttuessa. Dokumentin tulee tässä
vaiheessa sisältää ainakin seuraavat tiedot:

- Projektin nimi.
- Lyhyt kuvaus (1–2 kappaletta) projektin pääideasta, laajuudesta ja keskeisistä
  ominaisuuksista.
- Yleiskuvaus asiakasohjelmista (noin yksi kappale): onko käyttöliittymä
  graafinen ja mitä kirjastoja käytetään? Onko projektissa vain käyttäjän
  kanssa vuorovaikutuksessa oleva asiakasohjelma vai myös muunlaisia
  asiakasohjelmia eri rooleissa, esimerkiksi joidenkin objektien itsenäiseen
  ohjaamiseen?
- Dokumenttiin tulee myös palvelimen suunnittelukuvaus, mutta se laaditaan
  hieman myöhemmin, kun olemme käsitelleet tarkemmin palvelimen eri
  toteutusvaihtoehtoja.
- Alustava suunnitelma projektissa tarvittavista protokollaviesteistä ja niiden
  sisällöstä. Tässä vaiheessa suunnitelma saa olla luonnosmainen ja
  epätarkka. Protokollan yksityiskohdat tarkentuvat työn edetessä. Tavoitteena
  on, että saman aiheen parissa työskentelevät sovittavat protokollansa
  yhteensopiviksi tulevia yhteentoimivuustestejä varten.

**Osa 2:** Lähetä edellisessä tehtävässä toteuttamallasi asiakasohjelmalla
**TST**-viestejä vähintään kahteen kurssipalvelimen
**[konttilistassa](https://pronets1.dice.aalto.fi/)** näkyvään palvelimeen. Jos
löydät toteutuksestasi korjattavaa, tee tarvittavat korjaukset (ongelma voi
tosin olla myös toisessa päässä). Kirjaa raporttiin, mitä palvelininstansseja
testasit ja mitä havaitsit.

**Osa 3:** Toteuta **USR**-viesti, jolla rekisteröidään nimetty käyttäjä
palvelinyhteydelle. Tämä tulee lähettää ensimmäisenä viestinä asiakkaan
muodostaessa yhteyden palvelimeen. Palvelimen tulee vastata joko kaiuttamalla
viesti takaisin tai lähettämällä **ERR**-virheviesti, jos käyttäjää ei voida
rekisteröidä (esim. siksi että samanniminen käyttäjä on jo olemassa).
**TST**-viestiä lukuun ottamatta palvelimen ei tule hyväksyä muita viestejä
ennen kuin **USR**-viesti on vastaanotettu onnistuneesti ja käyttäjä on
rekisteröity. Toteuta sekä asiakas- että palvelinpuoli.

**Osa 4:** Toteuta **MSG**-viesti, joka lähettää annetun viestin kaikille annetun
kanavan käyttäjille. Voit olettaa, että käytössä on vain yksi kanava nimeltä
"**general**", johon kuuluvat kaikki palvelimeen yhdistetyt käyttäjät. Tuen
useille kanaville voi toteuttaa myöhemmin. Toteuta sekä asiakas- että
palvelinpuoli.

Kannattaa huomioida, että tässä vaiheessa toimintojen testaamiseksi riittää että
asiakasohjelma toimii vain komentorivillä. Jos graafisen käyttöliittymän
aloittaminen kiinnostaa jo nyt, ei siitäkään haittaa ole.

Kun olet toteuttanut yllä olevat kohdat, testaa ohjelmaasi paikallisesti
käyttäen **ainakin kahta samanaikaista asiakasohjelmaa** palvelimen kanssa. Kun
toteutus vaikuttaa toimivan, varmista että olet tallentanut muutoksesi
git-committeina ja työntänyt ne git-palvelimelle. Käynnistä sitten nykyinen
versio kurssin palvelimella **pronets1.dice.aalto.fi** käyttäen `/run-docker`-
rajapintaa edellisen tehtävän ohjeiden mukaisesti. Käytä tällä kertaa
protokollan tunnisteena "**base-2**".

Vastaa aiempaan tapaan lyhyesti myös seuraaviin kysymyksiin:

- Kuinka paljon aikaa käytit tehtävään?
- Mikä tehtävässä oli helppoa tai vaikeaa?
- Mitä työkaluja tai muita tietolähteitä käytit? Varsinkin jos käytit
  tekoälyavustimia, kerro miten käytit niitä ja olivatko ne hyödyllisiä.

</div>
