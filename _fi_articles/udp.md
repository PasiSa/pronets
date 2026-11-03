---
title: UDP ja reaaliaikainen tiedonsiirto
lang: fi-FI
translation_key: udp
---

Tähän asti olemme käyttäneet TCP:tä, joka on **yhteydellinen** protokolla ja
tarjoaa **luotettavan tavuvirtapalvelun** kahden päätepisteen välillä.
TCP-yhteys alkaa **kolmivaiheisella kättelyllä**, joka alustaa tarvittavan tilan
yhteyden molemmissa päissä. Luotettavuuden varmistamiseksi dataa puskuroidaan
yhteyden molemmissa päissä ja datasegmentit lähetetään uudelleen, jos
vastaanottaja ei kuittaa niitä ajoissa. TCP-lähettäjä säätelee myös
lähetysnopeuttaan verkon ruuhkautumisen välttämiseksi. Tämä takaa luotettavan
palvelun, mutta lisää tiedonsiirron viivettä. Seuraavaksi tutustumme hyvin
erilaiseen protokollaan: UDP:hen, joka ei lupaa luotettavaa tiedonsiirtoa, mutta
tarjoaa helpommin ennakoitavat viiveet, mistä on hyötyä sovelluksille, jotka
ovat herkempiä tiedonsiirron viiveille.

<div class="objectives-frame" markdown="1">

**Tämän moduulin tavoitteet:**

- Opit **tietosähkepistokkeiden käytön perusteet** ja niiden toiminnan erot
  tavuvirtapistokkeisiin verrattuna. Tietosähkepistokkeita käytetään tavallisesti
  viestintään UDP-protokollalla.

- Saat yksinkertaisen esimerkin avulla **peruskäsityksen reaaliaikaisen
  tiedonsiirron toteuttamisesta UDP:llä** clap-kirjastoa käyttäen.

- Ymmärrät, miten **osoitteenmuunnos (NAT) ja palomuurit** vaikuttavat
  kahden päätelaitteen väliseen viestintään ja miten niiden aiheuttamat
  vaikeudet toisen laitteen tavoittamisessa voidaan ratkaista.

</div>

## User Datagram Protocol ja tietosähkepistokkeet

**User Datagram Protocol, UDP ([RFC
768](https://datatracker.ietf.org/doc/html/rfc768))**, eroaa TCP:stä monin tavoin.
UDP ei takaa luotettavaa tiedonsiirtoa eikä ylläpidä päätepisteiden välillä
yhteyskohtaista tilaa. Siksi se ei myöskään ole yhteydellinen: UDP-lähettäjä voi
aloittaa datan lähettämisen heti, kun pistoke on luotu ja sidottu paikalliseen
osoitteeseen. UDP:n viestintämalli on **tietosähkepalvelu**: pistokkeeseen
lähetettyjen tietosähkeiden viestirajat säilyvät. Jos lähettäjä lähettää 1000
tavun datayksikön, vastaanottajan lukuoperaatio palauttaa 1000 tavua (ellei
tietosähke katoa verkossa). TCP:tä käytettäessä meidän piti toteuttaa
sovellukseen kehystysprotokolla (viestin alussa olevat pituus- ja tyyppiotsakkeet)
viestirajojen löytämiseksi. UDP:llä tätä ei tarvita: kun luemme seuraavan
tietosähkeen, tiedämme sen alkavan aina viestin alusta lähettäjän tarkoittamalla
tavalla.

Tosin kuin TCP, epäluotettavassa UDP:ssa ei ole kuljetuskerroksen kuittauksia
vastaanotetulle datalle. Jos lähettävä sovellus tarvitsee vahvistuksen viestin
vastaanottamisesta, vastaanottavan sovelluksen on lähetettävä kuittaus takaisin.
Koska UDP:ssä ei ole uudelleenlähetyksiä tai ruuhkanhallintaa, puskuroinnin
tarve on pienempi (eli viivettä on vähemmän), vaikka myös UDP-pistokkeissa
käytetään jonkin verran puskureita. UDP sopii siksi sovelluksiin, joissa viiveen
ennustettavuus tärkeää, ja joissa viive on mahdollisimman pieni, kuten
reaaliaikaisiin ääni- ja videokeskusteluihin tai verkkopeleihin.

Koska UDP ei ole yhteydellinen, viestinnän aloittamiseen ei tarvita
`connect()`-operaatiota, ja samasta UDP-pistokkeesta voidaan lähettää dataa
useisiin eri kohteisiin. Siksi **tietosähkepistoke** käyttää tavallisesti
`send_to()`-funktiota, joka saa lähetettävän datapuskurin lisäksi parametrina
kohdeosoitteen, johon tietosähke lähetetään. Jos alla oleva verkko tukee
yleislähetystä (broadcast) tai monilähetystä (multicast), tietosähkepistokkeita
ja UDP:tä voidaan käyttää myös monilähetykseen: tällöin tiettyyn
**monilähetysosoitteeseen** lähetetty paketti voidaan vastaanottaa useilla
laitteilla, jotka ovat rekisteröityneet vastaanottamaan kyseiseen osoitteeseen
lähetettyä liikennettä.

### Yksinkertainen esimerkki

Kurssin repositorion esimerkki
**[simple-udp](https://github.com/PasiSa/pronets/tree/main/examples/simple-udp/src/main.rs)**
havainnollistaa tietosähkepistokkeiden ja UDP:n käyttöä. Tiedoston lopussa oleva
`main()`-funktio on hyvin lyhyt: se parsii muutaman komentoriviparametrin
`clap`-kirjastoa käyttäen ja käynnistää parametrien perusteella joko palvelimen tai
asiakkaan. Molemmat on toteutettu omissa funktioissaan samassa ohjelmassa.

**Pistokkeen sitominen.** Asiakasfunktio aloittaa luomalla UDP-pistokkeen
`bind()`-kutsulla:

```rust
let socket = UdpSocket::bind("0.0.0.0:0")?;
```

Koska UDP ei ole yhteydellinen, `connect()`-kutsua ei tarvita. Näillä
parametreilla järjestelmää pyydetään valitsemaan paikalliseksi portiksi mikä
tahansa vapaa UDP-portti. Osoite `0.0.0.0` sitoo pistokkeen kaikkiin
paikallisiin IPv4-osoitteisiin. Myös TCP-pistokkeen `connect()`-kutsu tekee
paikallisen osoitteen ja portin valinnan automaattisesti, vaikka tällaista
`bind()`-funktiota ei erikseen kutsutakaan. Voisimme antaa
`bind()`-kutsulle myös tietyn IP-osoitteen ja portin, mikä onnistuisi myös
TCP-pistokkeelle, jos `bind()`-funktiota kutsutaan ennen `connect()`-kutsua.

Vaikka UDP-portit valitaan samalta etumerkittömien 16-bittisten lukujen
arvoalueelta kuin TCP-portit, niiden nimiavaruudet ovat erilliset. Jos siis
projektisi kuuntelee asiakasyhteyksiä TCP-portissa 1234, voit samalla sitoa
UDP-pistokkeen porttiin 1234 projektin esimerkiksi reaaliaikaista tiedonsiirtoa
varten.

**Datan lähettäminen.** Ohjelma lukee käyttäjältä rivin merkkijonoon ja
lähettää sen kohteeseen `send_to()`-funktiolla:

```rust
if let Err(err) = socket.send_to(input.trim().as_bytes(), &args.connect_addr) {
    eprintln!("Failed to send datagram to {}: {}", args.connect_addr, err);
    continue;
}
```

Kuten TCP-pistokkeita käytettäessä, merkkijono muunnetaan ensin tavuiksi
lähettämistä varten ja annetaan ensimmäisenä parametrina. Toisena parametrina
annetaan kohdeosoite, jota ei ole vielä määritetty. Kohdeosoite voidaan antaa
samassa muodossa kuin `connect()`-kutsussa, esimerkiksi merkkijonona kuten tässä.
Merkkijonosta jäsennetään varsinainen IP-osoite ja portti. Vaikka tässä
esimerkissä ei tehdä niin, jokaisessa seuraavassa `send_to()`-kutsussa voitaisiin
antaa eri kohdeosoite, vaikka käytössä olisi sama pistoke.

Jos lähetyksessä tapahtuu virhe, siitä ilmoitetaan ja siirrytään silmukan alkuun
odottamaan käyttäjän seuraavaa syötettä.

On hyvä huomata, että myös UDP:n `send_to()`-kutsu voi pysäyttää ohjeman
suorituksen, ellei käytössä ole ei-estävää siirräntää (esim. Mio-kirjaston
avaulla). Syy on sama kuin TCP:llä: käyttöjärjestelmä asettaa datan pistokkeen
lähetyspuskuriin, kunnes verkkolaitteen ajuri voi lähettää sen, ja puskuri voi
täyttyä jos sovellus lähettaa dataa paljon ja nopeasti. Koska UDP ei takaa
luotettavuutta eikä käytä ruuhkanhallintaa, tällaiset odotusajat ovat yleensä
lyhyempiä ja harvinaisempia kuin TCP:llä. Nopeasti lähettävä sovellus voi silti
joissakin tilanteissa tuottaa dataa nopeammin kuin käyttöjärjestelmän ydin ehtii
käsitellä sitä. Esimerkiksi joissain tilanteissa verkkolaite ei välttämättä ole
hetkellisesti valmis lähettämään uutta dataa, mikä saattaa aiheuttaa puskurien
täyttymistä ylemmillä protokollakerroksilla.

**Datan vastaanottaminen.** Lähetyksen jälkeen asiakas odottaa palvelimen
vastausta `match`-lausekkeessa:

```rust
let (size, src) = match socket.recv_from(&mut buf) {
    Ok(result) => result,
    Err(err) => {
        eprintln!("Failed to receive datagram: {}", err);
        continue;
    }
};
```

Koska pistoketta ei ole yhdistetty tiettyyn vastapäähän, myös vastaanottamiseen
tarvitaan eri funktio kuin aiemmin TCP:n tapauksessa. `recv_from()` lukee yhden
tietosähkeen ja palauttaa kahden arvon monikon: vastaanotettujen tavujen määrän
sekä osoitteen, josta tietosähke saapui (tässä `src`). Tietosähkeiden
viestirajat säilyvät vastaanotettaessa. Kuten lähetettäessä, samaa pistoketta
voidaan käyttää eri lähettäjiltä tulevien tietosähkeiden vastaanottamiseen.

`recv_from()`-kutsu odottaa, kunnes tietosähke saapuu. Tässä esimerkissä se
sopii hyvin, mutta monimutkaisemmassa sovelluksessa, jossa on useita
syötelähteitä, tarvitaan edelleen tavanomaisia rinnakkaisuuden
hallintamenetelmiä.

**Palvelimen toiminta** muistuttaa UDP:llä hyvin paljon asiakkaan toimintaa.
Pistokkeita ei jaeta passiivisiin ja aktiivisiin, eikä uusien yhteyksien
hyväksymiseen tarvita `accept()`-kutsua. Keskeiset erot ovat, että palvelin
sitoo pistokkeen tiettyyn UDP-porttiin, jotta muut laitteet tietävät, mihin
osoitteeseen ja porttiin data lähetetään, ja että vastaanotto ja lähetys tehdään
eri järjestyksessä. Palvelin ei lue käyttäjän syötettä, vaan lähettää saapuneet
tietosähkeet sellaisinaan takaisin `recv_from()`-kutsun palauttamaan
osoitteeseen.

Myös UDP-pistokkeelle voidaan kutsua `connect()`-funktiota tietyllä
IP-osoitteella ja portilla. Tällöin pistoke liitetään yhteen tiettyyn
vastapäähän. Sen jälkeen voidaan käyttää tavallisia `read`- ja
`write`-operaatioita, eikä muista osoitteista tulevia tietosähkeitä
vastaanoteta. Kuljetusprotokollan toimintaan `connect()`-kutsu ei vaikuta:
UDP:ssä ei edelleenkään tehdä kolmivaiheista kättelyä. Tästä on kuitenkin yksi
etu: jos tiedonsiirrossa tapahtuu virhe, esimerkiksi koska pakettia ei pystytä
toimittamaan annettuun IP-osoitteeseen, verkkolaite voi lähettää takaisin päin
**ICMP-virheilmoituksen**, vaikka varsinaista protokollatason tilallista
yhteyttä ei ole. Jos UDP-pistokkeelle on tehty `connect()`-kutsu,
käyttöjärjestelmä voi välittää tämän virheen pistokkeelle virhekoodina, koska
ICMP-viesti sisältää alkuperäisen paketin otsakkeet, sisältäen esimerkiksi
lähetysportin.

## Reaaliaikainen äänen suoratoisto

UDP-tiedonsiirron perinteinen käyttökohde on reaaliaikainen äänikeskustelu
kahden tai useamman osallistujan välillä. Kukin päätepiste ottaa mikrofonin
äänestä näytteitä, muuntaa ne datapaketeiksi ja lähettää ne tarkoitetulle
vastaanottajalle tai usealle vastaanottajalle. Jos viivettä on liikaa,
vuorovaikutteinen keskustelu vaikeutuu. Ääninäytteen toimittamisen sijaan voi
olla hyödyllisempää pudottaa liian myöhään saapuva näyte, jotta uudemmille
näytteille jää tilaa.

Viivettä voi syntyä monesta lähteestä. Pakettien siirtäminen verkossa aiheuttaa
viivettä, mutta sovellusohjelmoija ei yleensä voi vaikuttaa siihen kovin paljon.
Myös itse sovellus voi aiheuttaa viivettä eri syistä. Tavallisesti ääninäytteet
kerätään ensin vakionopeudella puskuriin, josta ne kopioidaan
UDP-tietosähkeisiin ja lähetetään verkkoon. Vastaanottopäässä sama tapahtuu
toisin päin: data tallennetaan puskureihin odottamaan, että äänentoistolaite
käyttää sen. Datan jonottaminen puskureissa lisää viivettä. Siksi puskurit eivät
saisi olla liian suuria: mitä suurempi puskuri on, sitä enemmän viivettä voi
syntyä.

### CPAL-kirjastoon perustuva esimerkki

**[audio-streaming](https://github.com/PasiSa/pronets/tree/main/examples/audio-streaming/src/main.rs)**-esimerkki
esittelee yksinkertaisen (mutta silti yli 400-rivisen) sovelluksen, joka käyttää
**[CPAL (Cross-Platform Audio Library)
-kirjastoa](https://crates.io/crates/cpal)** ääninäytteiden keräämiseen
äänilähteestä ja toistamiseen äänentoistolaitteella. Se käyttää lisäksi
[opus-kirjaston](https://crates.io/crates/opus) tarjoamaa **Opus-koodausta ([RFC
6716](https://datatracker.ietf.org/doc/html/rfc6716))** raakadatan koodaamiseen
ja purkamiseen sellaiseen muotoon että ääntä voidaan siirtää tehokkaasti
UDP-tietosähkeissä.

CPAL käyttää **syötön ja tulostuksen takaisinkutsufunktioita (callback)**
äänilaitteiden kanssa toimimiseen. _Syötön takaisinkutsufunktio_ asetetaan
rivillä 194 sulkeumana samaan tapaan kuin esimerkiksi säikeitä käytettäessä.
`input_callback()` palauttaa sulkeuman, jota CPAL kutsuu toistuvasti, kun
riittävästi dataa on saatavilla. Takaisinkutsufunktio kerää CPAL:lta saadut
näytteet `pcm_buffer`-puskuriin. Kun dataa on kertynyt 20 millisekunnin verran,
se kutsuu _Opus-kooderia_, joka koodaa datan `opus_packet`-puskuriin. Puskurin
sisältö lähetetään pääfunktiossa aiemmin avattuun UDP-pistokkeeseen. Tässä
käytetään pääfunktiossa aiemmin luotua **yhdistettyä UDP-pistoketta**, joten
`send`-funktiota voidaan käyttää ilman kohdeosoitetta.

Alla oleva kuva havainnollistaa esimerkin syöttö- ja tulostuspolkuja sekä
niiden vuorovaikutusta CPAL:n ja eri puskureiden kanssa.

![Ääniesimerkki](/images/udp-cpal.svg){: width="100%" .center-img }

Saapuvien UDP-pakettien äänen toistamiseen käytetään **rengaspuskuria** `ring`.
Saapuvat UDP-tietosähkeet vastaanotetaan erillisessä säikeessä. Rengaspuskuri
jaetaan `decoder_producer`-kahvaan, joka siirretään säikeen sisään, ja
`decoder_consumer`-kahvaan, jota **tulostuksen takaisinkutsufunktio** käyttää
pääsäikeestä. Rengaspuskuri on tietoliikenteessä yleinen tietorakenne: se pitää
kirjaa kohdasta, johon seuraava tietoalkio kirjoitetaan, ja kohdasta, josta
kuluttajan tulee seuraavaksi lukea. Rengasrakenne mahdollistaa datan tehokkaan
käsittelyn jatkuvana virtana ilman, että puskurille tarvitsee myöhemmin varata
muistia uudelleen.

Tulostuksen takaisinkutsufunktio on yksinkertainen: se tarkistaa, onko
puskurissa dataa, ja jos on, kopioi sen CPAL-kirjaston takaisinkutsun
parametrina antamaan äänen toistopuskuriin, josta kirjasto toistaa sen.
Alkiot poistetaan samalla rengaspuskurista. Jos puskuri on tyhjä, toistetaan
hiljaisuutta eli äänen toistopuskuriin kopioidaan nolla-arvoja.

Tietoliikennesäie lukee saapuvia tietosähkeitä silmukassa. Koska käytämme
yhdistettyä UDP-pistoketta, voimme käyttää `recv`-funktiota, joka ei palauta
tietosähkeen lähdeosoitetta, sillä tiedämme sen olevan aina sama.
Tietosähkeen hyötykuorma välitetään Opus-dekooderille, joka palauttaa purettuja
ääninäytteitä esittävän lukuarvovektorin. Arvot muunnetaan CPAL:n toistossa
käyttämiksi liukuluvuiksi välille -1.0–1.0 ja lisätään rengaspuskuriin, josta
takaisinkutsufunktio voi lukea ne.

Jos rengaspuskuri on täynnä, saapuva tietosähke pudotetaan. Rengaspuskuriin
varataan tilaa `sample_rate`-määrälle näytteitä eli noin yhden sekunnin verran
ääninäytteitä.

### Äänensiirron laadun parantaminen

Esimerkkisovelluksen äänenkäsittely on hyvin yksinkertaista. Jos UDP-tietosähke
katoaa verkossa tai tietosähkeet saapuvat määränpäähän väärässä järjestyksessä
(mikä ei ole harvinaista Internet-liikenteessä), ääneen tulee häiriöitä.
Opus-dekooderi ylläpitää purkamisen tilaa usean kehyksen ajan, joten häiriö voi
kestää pidempään kuin yhden tietosähkeen sisältämä ääni sellaisenaan.

Järjestyksen muuttumisen vaikutuksia voisi lieventää lisäämällä sovellukseen
**viivevaihtelupuskurin** (jitter buffer), jonka avulla sisään tulevia paketteja
viivästytetään hetki ja annetaan siten mahdollisuus verkossa viivästyneelle
datalle osua puskuriin oikealle paikalleen. Näin äänen toistoon tulee hieman
viivettä, mutta toiston laatu voi parantua. Väärässä järjestyksessä saapuvien
tietosähkeiden havaitseminen edellyttäisi UDP-tietosähkeisiin
protokollaotsakkeita, jotka sisältävät paketin järjestysnumeron,
lähetysaikaleiman tai molemmat. Opus osaa myös käyttää pakettihäviön
peittämisalgoritmia (packet-loss concealment), joka lieventää kadonneen paketin
aiheuttamaa kuulohavaintoon vaikuttavaa häiriötä. Tämä edellyttää, että
dekooderi saa tiedon paketin katoamisesta, mikä on mahdollista vain, jos
tietosähkeissä on järjestysnumerot.

IETF on määritellyt UDP:n päälle **Real-time Transport Protocol -protokollan
(RTP, [RFC 3550](https://datatracker.ietf.org/doc/html/rfc3550))** äänen ja
videon siirtoon. RTP lisää otsakkeen, joka sisältää järjestysnumeron, median
aikaleiman sekä tunnisteet lähteelle ja hyötykuorman formaatille. Jos se olisi
toteutettu, esimerkissämme RTP-otsake olisi kunkin UDP-tietosähkeen sisällä
ennen koodattua Opus-ääntä.

RTP ei itsessään takaa tiedon perillemenoa eikä toteuta viivevaihtelupuskuria.
Vastaanottava sovellus käyttää otsakkeen tietoja pakettien järjestämiseen,
toistohetken päättämiseen ja sen havaitsemiseen, milloin puuttuvaa ääntä on
peitettävä. RTP:n rinnalla käytettävä **RTP Control Protocol (RTCP)** tarjoaa
vastaanottoraportteja, jotka sisältävät tietoja muun muassa pakettihäviöstä ja
saapumisaikojen vaihtelusta. Lähettävä sovellus voivat käyttää näitä tietoja
laadun seurantaan ja lähetyksen formaatin mukauttamiseen vastaamaan verkon
kapasiteettia.

## Osoitteenmuunnos ja palomuurit

Ensimmäisessä moduulissa käsiteltiin lyhyesti erilaisia IP-osoitteita. Kun
viestitään Internetissä täytyy käyttää julkisia globaaleja IP-osoitteita.
32-bittisessä IPv4-osoiteavaruudessa globaaleja osoitteita on rajallinen määrä.
Saatavilla olevat osoitelohkot on jo jaettu verkko-operaattoreille, jotka
jakavat niitä edelleen asiakkailleen. Samalla Internetiin liitettyjen laitteiden
ja tarvittavien IP-osoitteiden määrä kasvaa mobiililaitteiden, IoT-laitteiden ja
datakeskusten virtuaalipalvelimien myötä.

Osoitteiden rajallisuuden vuoksi päätelaitteet käyttävät usein yksityisiä
IP-osoitteita. Niiden jakaminen ei edellytä maailmanlaajuista koordinointia,
vaan paikallisen verkon ylläpitäjä voi määrittää ne laitteille. Yksityisiä
osoitteita käyttäviä IP-paketteja ei kuitenkaan voida välittää Internetiin.
Siksi paikallisen yhdyskäytävän on korvattava IP-otsakkeen yksityinen osoite
julkisella IP-osoitteella ennenkuin se välitetään eteenpäin. Tätä kutsutaan
**osoitteenmuunnokseksi (Network Address Translation, NAT)**. Nykyään NAT on
hyvin yleinen: esimerkiksi useimmat WiFi-reitittimet käyttävät sitä.
Langattomaan verkkoon kytkettyjen laitteiden kokoelma vaihtelee eri aikoina, ja
osoitteet on jaettava dynaamisesti, mutta esimerkiksi kotikäyttäjälle
operaattori antaa yleensä vain yhden julkisen IP-osoitteen.

Tyypillisesti useat paikallisen verkon laitteet jakavat yhden Internetiin
näkyvän julkisen osoitteen. Yhdyskäytävän on siksi ylläpidettävä tietoa
tekemistään osoitteenmuunnoksista. Se säilyttää muistissaan muunnostaulua, jossa
ovat lähteen ja kohteen IP-osoitteet ja portit ennen muunnosta ja sen jälkeen.
Näin se voi tehdä johdonmukaisesti saman muunnoksen saman yhteyden myöhemmille
paketeille. Koska useat laitteet voivat jakaa saman julkisesti reititettävän
IP-osoitteen, myös porttinumero on joskus vaihdettava, koska kahdella
erillisellä yhteydellä ei voi olla samaa IP-portti - paria. Näin voidaan erottaa
kahden eri laitteen yhteydet, jotka käyttävät yksityisessä verkossa samaa
paikallista porttinumeroa mutta erillistä yksityistä IP-osoitetta.

Alla olevassa kuvassa kaksi laitetta on langattoman yhdyskäytävän takana ja
käyttää yksityisiä IP-osoitteita `192.168.1.2` ja `192.168.1.3`. Yhdyskäytävän
julkinen IP-osoite on `195.148.2.7`. Tässä esimerkissä molemmat laitteet avaavat
TCP-yhteyden palvelimeen, jonka julkinen IP-osoite on `128.4.64.10`, ja sattuvat
käyttämään paikallista TCP-porttia 5000. Tämä on mahdollista, koska laitteet
ovat erillisiä eivätkä tiedä toisistaan. Yhdyskäytävä korvaa lähtevien pakettien
lähdeosoitteen julkisella osoitteella `195.148.2.7`. Koska yhteyksillä olisi nyt
täsmälleen sama lähdeosoitteen ja -portin pari, toisen yhteyden
porttinumero on muutettava. Yhdyskäytävä tallentaa muunnokset muistiinsa, jotta
se voi tehdä käänteisen muunnoksen vastauspaketeille. Palvelimen näkökulmasta
yhteydet tulevat samasta osoitteesta ja samalta laitteelta. NAT siis piilottaa
paikallisen verkon rakenteen ja sen eri laitteet.

![Esimerkki osoitteenmuunnoksesta](/images/udp-nat.svg){: width="100%" .center-img }

Jos yhdyskäytävä käynnistettäisiin uudelleen (joko tarkoituksella tai
esimerkiksi sähkökatkon vuoksi), muunnostiedot katoaisivat ja kaikki
yhdyskäytävän kautta kulkevat yhteydet lakkaisivat toimimasta. Alkuperäisen
Internetin päästä päähän -suunnitteluperiaatteen mukaan näin ei pitäisi käydä,
koska paketteja välittävien laitteiden ei tulisi ylläpitää yhteyskohtaista
tilaa.

Osoitteenmuunnos on vaikeuttanut Internet-protokollien suunnittelua. Yksi
keskeisistä suunnitteluperiaatteista on ollut **päästä päähän -periaate (End to
end principle)**: tilan ylläpito ja siihen liittyvä monimutkaisuus jätetään
yhteyden päätelaitteiden kuljetuskerrokseen, ja verkko pidetään yksinkertaisena
ilman yhteyskohtaista tilaa. Tämä on ollut tärkeää Internetin skaalautuvuuden ja
toimintavarmuuden kannalta. Osoitteenmuunnoksen ja yhteyskohtaisten
muunnostietojen voidaan katsoa rikkovan tätä periaatetta, mikä on vaikuttanut
protokollien suunnitteluun.

Varhaiset Internet-protokollat, kuten tiedostonsiirtoon käytetty FTP, sekä **SIP
(Session Initiation Protocol, [RFC
3261](https://datatracker.ietf.org/doc/html/rfc3261))**, jolla muodostetaan ja
hallitaan esimerkiksi IP-puheluita, välittivät päätelaitteiden IP-osoitteita ja
TCP- tai UDP-portteja suorien yhteyksien muodostamista varten. Silloin kun
yhteyspolulla on NAT, tällaisia suoria vertaisyhteyksiä ei voi muodostaa
sellaisenaan, koska yksityisiä IP-osoitteita ei voi käyttää reitityksessä,
eivätkä ne näy yhteyden toiselle päälle.

Tarkemmin ottaen NAT:t aiheuttavat seuraavia käytännön ongelmia:

- Laite saa paikallisen IP-osoitteensa ja porttinsa selville `local_addr()`
  -funktiolla. Jos se välittää tämän osoitteen esimerkiksi SIP-viestissä ja
  yhteyden reitillä on NAT, vastaanottaja saa väärän osoitteen: todellinen
  globaalisti saavutettava osoite määritetään vasta NAT-yhdyskäytävässä.
  **Paikallinen laite ei voi suoraan tietää julkista osoitettaan ja porttiaan**
  ilman täydentäviä protokollamenetelmiä.

- Vaikka toinen osapuoli tietäisi julkisen osoitteen, yhteyttä ei yleensä voi
  **avata NAT-yhdyskäytävän julkiselta puolelta**, koska NATilta **puuttuu
  osoitteenmuunnokseen tarvittava tila**. Se syntyy tavallisesti vasta, kun
  yhteys tai UDP-istunto avataan yksityisen verkon sisältä.

- Koska NAT-yhdyskäytävä ylläpitää yhteyskohtaista tilaa, sen on myös voitava
  poistaa tarpeettomat tilatiedot. Yhteydellisessä TCP:ssä tämä on helpompaa,
  koska yhteyden avaamisesta ja sulkemisesta ilmoitetaan SYN- ja FIN-paketeilla.
  UDP:ssä vastaavaa signalointia ei ole. Siksi NAT-yhdyskäytävä ylläpitää
  ajastinta kullekin sen läpi kulkevalle vuolle ja **poistaa muunnostiedot,
  kun liikennettä ei ole ollut tiettyyn aikaan**. Jos sovelluksen toimintaan
  kuuluu pitkiä taukoja, sen on lähetettävä **säännöllisiä ylläpitoviestejä
  (keep-alive)** muunnostilan säilyttämiseksi. Tämä on tarpeen myös TCP:ssä,
  koska yhteys voi kadota ilman erillistä ilmoitusta esimerkiksi laitteen
  sammuessa tai käynnistyessä uudelleen. Myös NATin takana toimivien
  TCP-sovellusten on siksi lähetettävä tarvittaessa ylläpitoviestejä.

NAT-yhdyskäytävään voidaan kuitenkin määrittää kiinteitä porttiohjauksia,
joiden avulla yksityisen verkon palvelin voidaan tehdä saavutettavaksi ilman,
että yhteyskohtaista tilaa välttämättä tarvitaan. Tämä edellyttää pääsyä
NAT-asetuksiin, mikä on yleensä mahdollista vain järjestelmän ylläpitäjille.

Vaikka reitillä ei käytettäisi osoitteenmuunnosta, useimpien organisaatioiden
verkoissa on tilallinen **palomuuri**, joka estää ei-toivotun liikenteen pääsyn
paikalliseen verkkoon. Palomuuri ei muuta osoitteita tai portteja, mutta
yhteyskohtaisen tilan ylläpito aiheuttaa edellä kuvatun kaltaisia ongelmia.
Yleensä vain paikallisen verkon sisältä avatut yhteydet sallitaan, joten
palvelimen käynnistäminen yksityisessä verkossa edellyttää palomuurin asetusten
muuttamista. Lisäksi yhteyden tilaa on virkistettävä pitkien liikennetaukojen
aikana.

Yksi tapa mahdollistaa viestintä NATin takana olevien laitteiden välillä on
käyttää julkisessa verkossa sijaitsevaa **välityspalvelinta**, jolla on
julkisesti saavutettava IP-osoite. Molemmat päätelaitteet lähettävät
UDP-tietosähkeet välityspalvelimelle. Näin syntyvät NAT-muunnokset sallivat
palvelimen lähettää tietosähkeitä takaisin laitteille. Kaikki kommunikointi
tapahtuu välityspalvelimen kautta, joka välittää tietosähkeet varsinaiselle
kohteelle. Ratkaisu kasvattaa viivettä, koska kaikki välitettävä liikenne kulkee
välityspalvelimen kautta. Jonkun pitää myös tarjota tällainen välityspalvelin
käytettäväksi julkiseen Internetiin. **TURN ([RFC
8656](https://datatracker.ietf.org/doc/html/rfc8656))** on tähän tarkoitukseen
standardoitu protokolla.

Alla oleva kuva havainnollistaa välityspalvelimen käyttöä. Vertaisistunnossa,
kuten äänikeskustelussa, olisi tehokkainta, jos `host 1` ja `host 2` voisivat
viestiä suoraan keskenään. Koska molemmat ovat NATin takana, niiden on ensin
lähetettävä paketti julkiseen verkkoon, jotta yhdyskäytäviin syntyvät tarvittavat
NAT-muunnokset. Julkisessa Internetissä oleva välityspalvelin vastaanottaa
tietosähkeet ja välittää ne tarkoitetulle vastaanottajalle.

![Välityspalvelin](/images/udp-relay.svg){: width="100%" .center-img }

<div class="assignment-frame" markdown="1">

## Tehtävästä

Tähän moduuliin ei liity tehtävää. Onnea projektiin!

</div>
