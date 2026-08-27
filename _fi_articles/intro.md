---
title: Johdanto
lang: fi-FI
translation_key: intro
---

Tässä moduulissa käsitellään kurssilla tarvittavia tietoverkkojen perusteita ja
esitellään verkkotyökaluja jotka saattavat olla kurssina ikana hyödyksi.
Kurssilla oletetaan, että olet suorittanut jonkun tietoverkkojen peruskurssin,
kuten _ELEC-C7241 Tietokoneverkot_ tai _ELEC-C7420 Basic principles in
networking_. Siksi tämä osuus on vain lyhyt kertaus niistä käsitteistä, joita
tarvitaan kurssin myöhemmissä osissa.

<div class="objectives-frame" markdown="1">

**Moduulin tavoitteet:**

- **Kertaat TCP:n, UDP:n ja IP:n toimintaperiaatteet** sekä kuinka
  IP-osoitteiden määritellään päätelaitteille. Aiheet voivat olla tuttuja
  aiemmilta kursseilta, mutta hyvä kerrata asiat.

- **Opit nimipalvelun (Domain Name System, DNS) perusteet.** DNS on yksi
  Internetin tärkeimmistä palveluista, eikä Internet olisi käytännössä
  käyttökelpoinen ilman sitä.

- **Tutustut verkon analysoinnin perustyökaluihin:** **Netcatilla** lähetetään
  ja vastaanotetaan tietoa toisen verkkolaitteen kanssa, **Digillä** tutkitaan
  DNS-tietueita ja **Wiresharkilla** tarkastellaan verkkopakettien sisältöä ja
  eri protokollakerrosten otsakkeita.

- Kurssin työskentely tapahtuu Git-repositoriossa. Tässä moduulissa **opit
  Gitin perustoiminnot** ja Git-repositorion perustamisen.

</div>

## Yhteenveto kurssista

Kurssi jakautuu kahdeksaan eri aiheita käsittelevään moduuliin. Useimmissa
moduuleissa kehität asiakas–palvelinprojektiasi annetun teeman mukaisesti.
Tavallisesti tehtävässä toteutetaan uutta koodia tai muokkataan aiempaa
toteutusta. Lisäksi tehdään kirjallinen tehtävän, jossa käsitellään ja
analysoidaan pyydettyjä kohtia toteutuksessa. Koodin yksityiskohtia ei arvioida,
vaan arviointi kohdistuu kirjallisissa raporteissa esitettyyn pohdintaan ja
analyysiin.

Ensimmäisessä moduulissa käsitellään Internet-protokollan kahta versiota,
**IPv4:ää** ja **IPv6:ta**, kuljetuskerroksen protokollia **TCP** ja **UDP**
sekä kaikkien Internet-palvelujen toiminnalle olennaista **nimipalvelua (DNS)**.
Lisäksi tutustutaan yleisiin verkon diagnostiikka- ja analysointityökaluihin
sekä **Git-versionhallintaan**, jolla kurssin projekteja hallitaan. Harjoittelet
**Wiresharkin** ja **netcatin** käyttöä yksinkertaisessa HTTP-yhteydessä.

**Moduuli 2** alkaa **Rust-ohjelmointikielen** peruskäsitteistä. Rustia
käytetään kurssin esimerkeissä, ja sitä suositellaan myös tehtäviin. Toteutat
Rustilla yksinkertaisen TCP-asikassovelluksen joka tekee HTTP-pyynnön, sekä
pienen harjoitustehtävän, jossa kokeilaan yhteydenottoa kurssipalvelimelle.

**Moduulissa 3** toteutaan yksinkertainen TCP-palvelin Rustia käyttäen. Useiden
samanaikaisten asiakkaiden tehokkaaseen käsittelyyn tai suorituskykyyn ei vielä
keskitytä. Käsittelemme myös **Docker-konttien** perusteita. Palvelinohjelmasta
rakennetaan Docker-levykuva ja se rekisteröidään annetun rajapinnan
avulla kurssipalvelimelle, joka rakentaa ja käynnistää levykuvan julkisessa
IP-osoitteessa, annetussa portissa.

**Moduulissa 4** aloitetaan varsinainen projekti, jota kehitetään tehtävien
rinnalla kurssin loppuun asti. Voit valita projektin aiheen ja toteutuksen
yksityiskohtia, mutta kaikkien toteutusten on noudatettava moduulissa kuvattuja
protokollaperiaatteita ja yhteisiä protokollaviestejä. Näin eri projektien
yhteentoimivuutta voidaan testata, kun ne on otettu käyttöön Docker-kontteina
kurssipalvelimella.

**Moduuli 5** keskittyy toteutuksen testaamiseen sekä toiminnan ja suorituskyvyn
havainnointiin. Tutustumme Rustin testaustyökaluihin ja ohjelmiston toiminnan
analysoinnissa käytettäviin jäljitystyökaluihin.

**Moduulissa 6** käsitellään edistyneempiä tapoja palvella tehokkaasti suurta
määrää samanaikaisia asiakasistuntoja. Opit toteuttamaan monisäikeisen
palvelimen ja käsittelemään yhteiskäyttöistä tietoa turvallisesti rinnakkaisessa
ympäristössä. Tutustut Rustin asynkroniseen ohjelmointimalliin ja suosittuun
**Tokio**-kirjastoon.

**Moduulissa 7** projekteihin lisätään tietoturva. Tästä eteenpäin kaikkien
projektien on käytettävä **Transport Layer Security (TLS)** -suojausta asiakkaan
ja palvelimen välisessä viestinnässä sekä kurssipalvelimen rajapintojen kanssa.
Opimme käyttämään **JSON Web Token** -tunnisteita todentamiseen ja
käyttöoikeuksien hallintaan.

**Moduuli 8** käsittelee **UDP:tä ja reaaliaikaista viestintää**. Lisäät
ohjelmistoosi TCP:n sijasta UDP:tä käyttävän reaaliaikaisen osan. Lisäksi
käsittelemme muita aiheeseen liittyviä edistyneitä aiheita.

## Internet Protocol (IP)

Verkkolaitteet liittyvät Internetiin IP-protokollan avulla. Protokollasta on
kaksi versiota: vanha **IPv4** on edelleen laajasti käytössä, mutta sillä on
rajoitteensa: IPv4-osoitteessa on vain 32 bittiä eli noin neljä miljardia
mahdollista arvoa, mikä ei riitä nykyisiin tarpeisiin, jos osoitteta jaettaisiin
jokaiselle laitteelle alkuperäisen tarkoituksen mukaisesti. Puutteiden
korjaamiseksi kehitettiin myöhemmin **IPv6**, jossa osoite kasvatettiin
128-bittiseksi muiden parannusten ohella. IPv6:n Käyttöönotto on ollut hidasta, koska
päätelaitteiden, reitittimien ja muiden verkon ytimessä olevien laitteiden
siirtäminen uuteen protokollaan on vaikeaa. Käytännössä IPv4- ja IPv6-osoitteet
toimivat rinnakkain: IPv6:ta tukeva laite voi käyttää joissakin yhteyksissä
IPv4:ää, jos verkkopolku tai toinen osapuoli ei tue uudempaa protokollaa.
[Cloudflaren Radar-palvelun](https://radar.cloudflare.com/) mukaan noin 40 %
Internet-liikenteestä käyttää tällä hetkellä IPv6:ta, mutta alueelliset erot
ovat suuria.

IP ja muut Internet-protokollat määritellään RFC-dokumenteissa, joihin tässä
materiaalissa viitataan eri kohdissa. RFC-määrityksistä vastaa
**[Internet Engineering Task Force (IETF)](https://www.ietf.org/)**. IETF:n
standardointiprosessi on avoin ja julkinen, johon kuka tahansa voi osallistua.

### Pakettien lähettämisestä

Tietokoneen tai mobiililaitteen käyttöjärjestelmä jakaa verkkosovellusten
lähettämät ja vastaanottamat viestit paketeiksi. Paketit toimitetaan verkossa
toisistaan riippumatta, ja niihin kohdistuu jonotusta ja muita viiveitä, ja ne
voivat kadota matkalla, yleensä verkon ruuhkautumisen vuoksi.

IP-paketin enimmäiskoosta käytetään termiä **Maximum Transmission Unit (MTU)**.
MTU riippuu useimmiten alla olevasta linkkiteknologiasta. IEEE 802 -perheen
lähiverkkoprotokollat, kuten Ethernet (802.3) ja Wi-Fin eri sukupolvet (802.11),
ovat olleet pitkään suosittuja. Ne käyttävät 1500 tavun MTU:ta, josta on siksi
tullut hyvin yleinen kaikessa IP-viestinnässä. Nykyisten sovellusten
tietomääriin ja nykyisiin siirtonopeuksiin suhteutettuna pakettikoko on pieni,
joten verkkoon liitetyn tietokoneen on usein käsiteltävä tuhansia paketteja
hyvin lyhyessä ajassa, kuten pian nähdään Wiresharkin kanssa.

**Internet Control Message Protocol (ICMP)** välittää diagnostiikka- ja
virheilmoituksia, kuten ”Packet too Big”, jos verkkopolulla on linkki joka ei
pysty toimittamaan halutun kokoista pakettia, tai ”Destination unreachable”, jos
pakettia ei voida toimittaa kohteeseen. Myös **ping**-sovellus käyttää ICMP:tä
kohteen saavutettavuuden testaamiseen ja edestakaisen viiveen mittaamiseen.
_Ping_ lähettää sarjan _ICMP Echo Request_ -viestejä, joihin vastaanottaja
vastaa _ICMP Echo Response_ -viesteillä.

### IP-osoitteista

IP-paketin protokollaotsakkeessa on IPv4- tai IPv6-lähde- ja kohdeosoite.
Osoitteita on käyttötarkoituksensa perusteella erilaisia. Internetiin
liitetyllä laitteella voi olla samanaikaisesti useita IP-osoitteita:

- Jokaisella verkkolaitteella on oma IP-osoite.
  Langattomassa laitteessa voi esimerkiksi olla yhden operaattorin 5G-liittymä
  ja mahdollisesti toisen operaattorin Wi-Fi-liittymä. Niillä on eri
  IP-osoitteet, ja niiden kautta lähetetyt paketit kulkevat eri reittejä kohteeseen.
- Virtuaalikoneilla ja konteilla on **virtuaalisia verkkorajapintoja**, joilla on
  loogisesti omat IP-osoitteensa. Osoitteet annetaan usein yksityisestä
  osoiteavaruudesta: niitä ei voi reitittää sellaisenaan Internetiin, vaan ne on
  matkan varrella **muunnettava** maailmanlaajuiseksi IP-osoitteeksi.
- Laitteella voi olla yksityisiä IP-osoitteita esimerkiksi organisaation
  sisäiseen viestintään, ja samalla kuitekin globaali IP-osoite
  Internet-kommunikaatioon.
- Jos laite tukee sekä IPv4:ää että IPv6:ta. Sille on tyypillisesti määriteltu
  kummankin tyypin osoitteet.

IPv4-osoite merkitään tavallisesti neljänä pisteillä erotettuna 8-bittisenä
desimaalilukuna, esimerkiksi **151.101.245.91** koskien palvelielle
_www.aalto.fi_. IPv6-osoite esitetään kaksoispisteillä erotettuna 16-bittisten
heksadesimaalilukujen sarjana, esimerkiksi **2a04:4e42:3a::347** samalle
palvelimelle. Tämä IPv6-osoite vastaa osoitetta
**2a04:4ee4:003a:0000:0000:0000:0000:0347**, mutta peräkkäiset nollat on sovittu
lyhennettäviksi kahdella kaksoispisteellä. Aallon verkkosivuja voi siis käyttää
sekä IPv4:llä että IPv6:lla.

IP-osoite jakautuu kahteen osaan. Merkitsevimmät bitit muodostavat
**verkkoetuliitteen**, joka on yhteinen kaikille saman lähiverkon laitteille.
Vähiten merkitsevät bitit erottavat verkon laitteet toisistaan. Jokaisella
laitteella on oltava eri osoite. **Classless Inter-Domain Routing (CIDR)**
ilmaisee verkkoetuliitteen ja sen pituuden esimerkiksi muodossa
**164.90.208.0/20**. Näin voidaan päätellä, että osoitteet **164.90.208.10** ja
**164.90.209.14** kuuluvat samaan verkkoon eikä niiden välistä liikennettä
tarvitse välittää verkkoreitittimelle.

**Dynamic Host Configuration Protocol (DHCP, [RFC
2131](https://datatracker.ietf.org/doc/html/rfc2131) /
[Wikipedia](https://en.wikipedia.org/wiki/Dynamic_Host_Configuration_Protocol))**
on tapa jakaa IP-osoitteita lähiverkon laitteille. DHCP-palvelin pitää kirjaa
vapaista osoitteista ja antaa vapaan osoitteen sitä pyytävälle laitteelle. Kun
uusi laite liittyy verkkoon, se selvittää DHCP-kyselyllä IP-osoitteensa ja
esimerkiksi paikallisen DNS-palvelimen osoitteen. IPv6 käyttää usein **tilatonta
osoitteiden automaattista määritystä (stateless address autoconfiguration, [RFC
4862](https://datatracker.ietf.org/doc/html/rfc4862) /
[Wikipedia](<https://en.wikipedia.org/wiki/IPv6#Stateless_address_autoconfiguration_(SLAAC)>))**.
Se perustuu oletukseen, että linkkikerroksen MAC-osoitteet, tavallisesti 48
bittiä, ovat lähiverkossa yksilöllisiä. MAC-osoitteen avulla muodostetaan
IPv6-osoitteen 64-bittinen laiteosa. Verkko-osa ja reitittimen IPv6-osoite
saadaan **IPv6 router advertisement** -viestistä. IPv6:ssa sekä verkko-osa että
laite- eli liitäntätunniste ovat tavallisesti 64-bittisiä.

IPv4- ja IPv6-osoitteita on erilaisia:

- **Konekohtaiset eli loopback-osoitteet (IPv4: 127.0.0.1, IPv6: ::1)** on
  tarkoitettu saman tietokoneen sovellusten väliseen viestintään. Ne ovat
  hyödyllisiä erityisesti paikallisessa kehityksessä ja testauksessa. Näihin
  osoitteisiin lähetetyt paketit eivät poistu tietokoneesta edes lähiverkkoon.

- **Yksityisiä osoitteita (IPv4: 10.0.0.0/8; 172.16.0.0/12; 192.168.0.0/16;
  IPv6: fc00::/7)** käytetään lähiverkoissa, kuten kodin Wi-Fi-verkossa,
  toimiston sisäverkossa sekä virtuaalikoneiden ja konttien virtuaaliverkoissa.
  Näillä osoitteilla varustettuja paketteja ei voi reitittää Internetiin, vaan
  ne on tarkoitettu lähiverkon laitteiden väliseen viestintään. Osoitteet ovat
  suosittuja, koska niitä voidaan jakaa lähiverkoille vapaasti, ilman että
  osoitetta tarvitsisi varata operaattorilta. Verkkoreititin, esimerkiksi kodin
  Wi-Fi-tukiasema, muuntaa tavallisesti Internetiin lähtevien pakettien
  yksityisen osoitteen julkiseksi osoitteeksi.

- Suurin osa muista osoitteista on **maailmanlaajuisia Internet-osoitteita**,
  joita reitittimet voivat välittää mille tahansa Internetin laitteelle. Ne on
  saatava verkko-operaattorilta.

Lisäksi on esimerkiksi ryhmä- ja yleislähetykseen tarkoitettuja osoitteita,
joita tällä kurssilla ei tarvita. Lisätietoja löytyy Wikipedian artikkeleista
[IPv4](https://en.wikipedia.org/wiki/IPv4) ja [IPv6-osoitteet](https://en.wikipedia.org/wiki/IPv6_address).

## Transmission Control Protocol (TCP)

IP:n päällä käytetään tavallisimmin **TCP-protokollaa ([RFC
9293](https://datatracker.ietf.org/doc/html/rfc9293) /
[Wikipedia](https://en.wikipedia.org/wiki/Transmission_Control_Protocol))**
luotettavan viestikavanavan (tai "putken") muodostamiseen kahden
Internet-laitteen välille. TCP tarjoaa ylemmille protokollakerroksille
abstraktion **luotettavasta tavuvirrasta**. Se **ei säilytä sovelluksen
lähettämien viestien rajoja**, mikä on huomioitava sovelluksen viestintäoperaatioiden
suunnittelussa. Kun sovellus lähettää tietoa jollakin pistokerajapinnan
lähetystoiminnolla, tieto kopioidaan ensin käyttöjärjestelmän pistokkeen
lähetyspuskuriin. Käyttöjärjestelmä jakaa tiedon paketeiksi ja käsittelee ne
TCP:n sääntöjen mukaisesti.

TCP on kahden päätepisteen välinen yhteydellinen protokolla. Asiakkaan on ensin
avattava yhteys määrättyyn IP-osoitteeseen ja TCP-porttiin. Yhteyden toinen pää
on **palvelin**, joka kuuntelee asiakkaiden saapuvia TCP-yhteyksiä tunnetussa
IP-osoitteessa ja portissa. Yhteys alkaa asiakkaan käynnistämällä
**kolmivaiheisella kättelyllä**, ja varsinainen viestintä voi alkaa vasta kättelyn
valmistuttua. Sen jälkeen kumpikin osapuoli voi lähettää tietoa itsenäisesti,
vaikka tavallisesti asiakas aloittaa keskustelun, kuten HTTP:ssä.

16-bittinen TCP-portti määritetään IP-osoitteen tavoin yhteyden molemmille
päille. Portti erottaa laitteiden väliset TCP-yhteydet toisistaan ja ohjaa
paketit oikealle pistokkeelle ja sovellukselle. Palvelimen portti toimii myös
Internet-palvelun tunnisteena. Portit 80 ja 443 on esimerkiksi varattu
suojaamattomalle ja suojatulle HTTP:lle eli websisällön siirrolle, ja porttia 25
on käytetty sähköpostia välittävälle SMTP:lle (salaamattomana). Porttivarauksia
hallinnoi **[Internet Assigned Numbers Authority
(IANA)](https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml)**.
Kun tarkastellaan esim. Wiresharkilla kaapattuja paketteja, TCP-paketeissa
palvelinpään portti kertoo yleensä käytetyn palvelun. Asiakkaan portti näyttää
satunnaiselta ja on tavallisesti automaattisesti valittu yli 48000:n oleva
numero. Asiakastoteutus ei yleensä valitse paikallista porttia, vaan
käyttöjärjestelmä valitsee vapaan portin sovelluksen puolesta.

Pakettien toimittaminen asiakkaan ja palvelimen välillä saattaa kestää. Fyysisen
etenemisviiveen sekä reitittimien käsittely- ja jonotusviiveiden lisäksi TCP:n
lähettäjä rajoittaa lähetysnopeutta ruuhkan- ja vuonhallinta-algoritmeillaan.
Koska TCP takaa järjestetyn tietovirran luotettavan toimituksen, vastaanottaja
ei toimita kadonnutta pakettia seuraavaa tietoa sovellukselle ennen kuin
puuttuva osa saadaan lähetettyä uudestaan ja toimitettua perille.
Vastaanottavalle sovellukselle tämä voi näkyä vaihtelevina viiveinä ja
äkillisinä taukoina. Sovelluksen suunnittelijan on otettava myös tämä huomioon.

## User Datagram Protocol (UDP)

**User Datagram Protocol (UDP)** on yksinkertainen IP:n päällä toimiva
kuljetusprotolla, jolla lähetetään valitun kokoisia tietopaketteja
(datagrammeja) kohteeseen. UDP:tä käsitellään tarkemmin myöhemmin kurssilla,
mutta seuraavassa lyhyt yleiskuvaus. UDP käyttää TCP:n tavoin 16-bittisiä lähde-
ja kohdeportteja viestintäistuntojen erottamiseen. TCP:hen verrattuna UDP on
hyvin yksinkertainen: se on epäluotettava, yhteydetön ja tilaton eikä takaa
tiedon toimittamista. Sovellus voi lähettää paketteja haluamallaan tahdilla,
mutta ei tiedä, saapuvatko ne perille, ellei vastaanottava sovellus kuittaa
toimistusta jollain tapaa. TCP:stä poiketen UDP säilyttää sovellustason
viestirajat. UDP sopiikin kevyisiin signalointitarkoituksiin, joissa
luotettavuus ei ole niin tärkeää, sekä ääni- ja videoneuvottelujen tai
verkkopelien kaltaiseen reaaliaikaiseen toimittamiseen, jossa vaihteleva viive
on yksittäistä tietohäviötä haitallisempaa. Yhteydettömyys mahdollistaa myös
IP:n yleis- ja ryhmälähetykset, joissa yhdellä paketilla on monta vastaanottajaa
eikä lähettäjä välttämättä tiedä ketkä kaikki viestin vastaanottavat. Tästä on
hyötyä esimerkiksi kun halutaan kysyä onko tietty palvelu (esim. tulostin)
löytyvillä jossain päin lähiverkkoa, ja mikä sen IP-osoite on.

## Domain Name System (DNS)

Yleisin UDP:tä käyttävä sovellus on nimipalvelu, **Domain Name System (DNS)**
([Wikipedia](https://en.wikipedia.org/wiki/Domain_Name_System)), joka käyttää
UDP-porttia 53. DNS on hierarkkinen nimitietokanta, joka liittää verkkotunnukset
IPv4- tai IPv6-osoitteisiin. Erityyppiset **resurssitietueet** voivat lisäksi
kertoa autoritäärisen nimipalvelimen (NS), ensisijaiseen (canonical) nimeen tai
nimialiakseen (CNAME), verkkotunnuksen sähköpostipalvelimeen (MX) ja monia muita
verkon resursseja. Teknisesti DNS on erillinen UDP:n ja IP:n päällä toimiva
sovellus, mutta käytännössä se on niin olennainen osa verkkosovelluksia, että
monet verkon ohjelmointirajapinnat ottavat sen annettuna.

Kun asiakassovellus haluaa ottaa yhteyden verkkopalvelimeen, se ei tavallisesti
tiedä palvelimen IP-osoitetta. Ennen TCP-yhteyden avaamista järjestelmän on
tehtävä DNS-kysely, jossa ilmoitetaan kohteen nimi, esimerkiksi
**www.aalto.fi**, ja haluttu tietuetyyppi. **A-kysely** pyytää nimeä vastaavaa
IPv4-osoitetta ja **AAAA-kysely** IPv6-osoitetta. Nykyaikaisissa
verkkorajapinnoissa nimikysely on usein yhdistetty samaan funktioon yhteyden
avaamisen kanssa, vaikka kyse on erillisestä toiminnosta ja viestinvaihdosta.

Sekä IPv4:ää että IPv6:ta tukeva järjestelmä voi tehdä yhden kyselyn kummallekin
osoiteperheelle ja käyttää jotakin saatua osoitetta. Yksi kysely voi myös
palauttaa useita IPv4- tai IPv6-osoitteita. Tämä parantaa toimintavarmuutta
palvelimen ollessa tilapäisesti poissa käytöstä ja mahdollistaa kuormantasauksen.

DNS on hajautettu ja hierarkkinen järjestelmä, jonka tiedot on toisinnettu
useille palvelimille. **Juurivyöhyke** sisältää **ylätason verkkotunnukset
(Top-level domaun, TLD)**, kuten _.fi_ ja _.com_. Se on hajautettu eri puolilla
maailmaa oleville juuripalvelimille, joilla on kunkin ylätason verkkotunnuksen
autoritääristen nimipalvelimien **NS-resurssitietueet**. Nämä nimipalvelimet
selvittävät seuraavan verkkotunnustason autoritäärisen nimipalvelimet, ja tätä
ketjua jatketaan kunnes varsinainen IP-osoite tai muu haluttu resurssitietue
löytyy.

Hierarkkinen selvitys aiheuttaisi viivettä ja kuormittaisi juuripalvelimia, jos
se tehtäisiin jokaisella kerralla alusta asti. Siksi nimiä tallennetaan
välimuisteihin kyselypolun varrella, ja usein yleisimmin viitattujen nimien
vastaus saadaan läheiseltä nimipalvelimelta. DNS:n **resurssitietueissa** on
tämän vuoksi myös elinaika, joka määrää, kuinka kauan tietuetta saa säilyttää
välimuistissa.

Nimen sijainti nimihierarkiassa ei välttämättä liity mitenkään IP-osoitteen
sijaintiin verkon topologiassa. Esimerkiksi _www.aalto.fi_ palauttaa aliaksen
(CNAME-tietueen) _dualstack.n.sni.global.fastly.net_, joka puolestaan palauttaa
A-kyselyllä IPv4-osoitteen _151.101.245.91_ tai AAAA-kyselyllä IPv6-osoitteen
_2a04:4e42:3a::347_. Tästä voi nähdä, että Aallon verkkosivuja ylläpitää ulkoinen
palvelu, eikä palvelin välttämättä ole Aallon kampukselta eikä _.fi_-päätteestä
huolimatta välttämättä edes Suomessa.

Alla oleva kuva havainnollistaa miten DNS-kysely etenee ja sitä, miksi
vastauksen saaminen voi joskus kestää. Kuva on blogikirjoituksesta “[How DNS
Resolution
Works](https://dev.to/swadesh_chatterjee_b35563/how-dns-resolution-works-55jm)”.

![DNS-selvitys](https://media2.dev.to/dynamic/image/width=800%2Cheight=%2Cfit=scale-down%2Cgravity=auto%2Cformat=auto/https%3A%2F%2Fdev-to-uploads.s3.amazonaws.com%2Fuploads%2Farticles%2Frklsx8po24i2biaoscvm.png){: width="90%" .center-img }

Asiakaslaite lähettää yleensä DNS-kyselyn paikalliselle DNS-palvelimelle.
Paikallinen DNS-palvelin aloittaa nimihierarkian selvittämisen ylätason
verkkotunnuksesta. Ensin kysytään tunnetulta juuripalvelimelta ylätason
verkkotunnuksen tietoja. Sen jälkeen kyseisen TLD:n nimipalvelimelta selvitetään
pyydetyn verkkotunnuksen autoritäärisen nimipalvelimen osoite. Lopuksi
paikallinen DNS-palvelin saa varsinaisen palvelimen IP-osoitteen ja toimittaa
sen asiakaslaitteelle. Välimuistien ansiosta yleisten nimien kohdalla osa
vaiheista voidaan ohittaa.

## Verkon analysointityökalut

Seuraavaksi tutustutaan muutamaan työkaluun, joista on hyötyä verkon toiminnan
analysoimisessa: **Digillä** tutkitaan DNS-tietueita, **Netcatilla** lähetetään
ja vastaanotetaan tietoa TCP:n tai UDP:n välityksellä ja **Wiresharkilla**
kaapataan ja tutkitaan verkkosovellusten lähettämiä paketteja.

Dig ja Netcat ovat komentorivityökaluja, ja ne voi Linuxissa tarvittaessa
asentaa järjestelmän paketinhallinnalla. Mac-koneilla niitä käytetään
**Pääte**-sovelluksessa, ja ne sisältyvät vakioasennukseen. Windowsissa
suositeltavaa asentaa **Windows Subsystem for Linux (WSL)**, joka tarjoaa
Linux-virtuaalikoneen ja vastaavat työkalut Windows-käytttäjille. Wireshark on
graafinen sovellus, joka on asennettava erikseen Linuxiin, macOS:ään tai
Windowsiin.

### Dig-työkalu

**Dig** on DNS-kyselyjen tekemiseen tarkoitettu komentorivityökalu. Se näyttää
DNS-palvelimen palauttamat tietueet sekä tietoja kyselyoperaatiosta, kuten
siihen käytetty aika. Tulosteen olennaisin osa on yleensä `ANSWER SECTION`,
jossa luetellaan löydetyt resurssitietueet ja niiden elinajat sekunteina (time-to-live, TTL).

Yksinkertaisimmillaan komennolle annetaan verkkotunnus, sekä tarvittatessa
haluttu tietuetyyppi. Seuraavassa tehdään esimerkiksi IPv4-kysely nimelle
`www.aalto.fi` ja näytetään yksi mahdollinen vastaus:

```bash
$ dig www.aalto.fi A

; <<>> DiG 9.10.6 <<>> www.aalto.fi A
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 16279
;; flags: qr rd ra; QUERY: 1, ANSWER: 2, AUTHORITY: 0, ADDITIONAL: 1

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 1232
;; QUESTION SECTION:
;www.aalto.fi.			IN	A

;; ANSWER SECTION:
www.aalto.fi.		373	IN	CNAME	dualstack.n.sni.global.fastly.net.
dualstack.n.sni.global.fastly.net. 27 IN A	199.232.173.91

;; Query time: 8 msec
;; SERVER: 192.168.0.1#53(192.168.0.1)
;; WHEN: Thu Jul 30 18:33:09 EEST 2026
;; MSG SIZE  rcvd: 104
```

Tuloste näyttää UDP-paketissa saadun DNS-vastauksen sisällön:

- Viestin tunniste oli 16279. Kyselyosassa on yksi resurssitietue ja
  vastausosassa kaksi. DNS-vastaus toistaa myös kyselyn, koska protokolla on
  tilaton.
- Ensimmäinen tietue on tyyppiä **CNAME**. Se ilmaisee palvelimen **ensisijaisen
  DNS-nimen**, joka todellisuudessa tarjoaa _www.aalto.fi_:n sisällön. Nimestä
  näkyy, että verkkosisällön jakaja on [Fastly](https://www.fastly.com/) -
  niminen yritys (nykypäivänä on yleistä, että sisällönjakopalvelu on
  ulkoistettu siihen erikoistuneelle yritykselle). Tietueen elinaika on 373
  sekuntia. DNS-tietueita tallennetaa välimuistiin varsinaisten kyselyviestien
  vähentämiseksi, ja välimuistit käyttävät tätä tietoa.
- Toinen tietue on palvelimen _dualstack.n.sni.global.fastly.net_ varsinainen
  IPv4-osoite, jonka elinaika on 27 sekuntia. Elinaika on lyhyempi, koska
  operaattorit tasaavat usein datakeskustensa kuormaa eivätkä halua osoitteiden
  säilyvän välimuisteissa liian pitkään. Puoli minuuttia myöhemmin tehty kysely
  voisi palauttaa toisen Aallon verkkosisältöä tarjoavan datakeskuspalvelimen
  osoitteen.

Vastausosassa voi olla myös useita IP-osoitetietueita. Ne antavat yhteyttä
avaavalle asiakkaalle varavaihtoehtoja ja parantavat toimintavarmuutta, jos
ensimmäinen osoite ei jostain syystä vastaa.

Lopuksi näytetään yleisiä tietoja vastauksesta: se saapui kahdeksassa
millisekunnissa yksityisen IP-osoitteen _192.168.0.1_ UDP-portista 53, kuten
DNS:ltä sopii odottaa. Tässä tapauksessa kyseessä on allekirjoittaneen
kotireititin, joka käyttää kodin laitteille yksityistä IP-verkkoa. Tämä on hyvin
tavallista.

Vastaava IPv6-osoitteen kysely tehdään näin:

```bash
$ dig www.aalto.fi AAAA
```

Digillä voi tutkia myös muita kuin laitteiden osoitteisiin liittyviä tietueita.
`MX`-kysely listaa verkkotunnuksen sähköpostipalvelimet ja `NS`-kysely sen
toimivaltaiset nimipalvelimet. Nämä eivät kuitenkaan ole tällä kurssilla yhtä
olennaisia.

```bash
dig aalto.fi MX
dig aalto.fi NS
```

### Netcat

**Netcat** (lyhyesti `nc`) on komentorivityökalu. Se avaa pistokkeen
ja välittää käyttäjän syötteen pistokkeeseen sekä pistokkeesta saadun tiedon
käyttäjälle. Netcat voi avata sekä asiakas- että palvelinpistokkeita TCP:lle ja
UDP:lle, joten se sopii hyvin verkko-ohjelmistojen testaamiseen ja
debugaukseen.

Seuraava komento avaa TCP-yhteyden palvelimen _www.aalto.fi_ HTTP-porttiin 80
ja lähettää yksinkertaisen HTTP-pyynnön:

```bash
$ nc www.aalto.fi 80
GET / HTTP/1.1
Host: www.aalto.fi
Connection: close

```

Kun komento ja sen argumentit on kirjoitettu ensimmäiselle riville, _netcat_
selvittää ensin palvelimen _www.aalto.fi_ IP-osoitteen ja avaa TCP-yhteyden sen
porttiin 80, jossa käytetään salaamatonta HTTP:tä. Sen jälkeen kaikki
päätteeseen kirjoitettu syöte lähetetään palvelimelle. Tässä tehdään HTTP 1.1
-standardin mukainen GET-pyyntö polkuun `/`. HTTP-otsake _Host_ kertoo
verkkopalvelimelle, että haluamme sivuston _www.aalto.fi_. Yksi palvelinkone voi
tarjota virtuaalisesti useita verkkosivustoja, kuten Fastly tekee, joten tieto
tarvitaan. Otsake _Connection_ pyytää palvelinta sulkemaan yhteyden vastauksen
jälkeen. Otsakkeiden jälkeen tarvitaan yksi tyhjä rivi eli pelkkä Enterin
painallus. Tämän jälkeen HTTP-vastauksen pitäisi näkyä päätteessä.

Netcat-palvelin käynnistetään tietyssä portissa `-l` - komentorivioptiolla:

```bash
nc -l 6000
```

Komento avaa porttiin 6000 saapuvia yhteyksiä kuuntelevan
TCP-palvelinpistokkeen, ja kun yhteys on muodostettu, jakaa komentorivin
syötteen pistokkeeseen edellä kuvatun mukaisesti.

Kun avaat toisen pääteikkunan ja yhdistät localhost-osoitteen porttiin 6000,
voit aloittaa asiakas- ja palvelinsocketin välisen viestinnän. Kokeilepa: yhteen
ikkunaan kirjoitetun tekstin pitäisi siirtyä toiseen ikkunaan.

```bash
nc 127.0.0.1 6000
```

Oletusarvoisesti käytetään TCP:tä, mutta kometorivioptiolla `-u` saman voi tehdä
TCP:n sijasta UDP:llä.

### Wireshark

**Wireshark** on verkon analysointityökalu, joka kaappaa kaikki verkkolaitteen
läpi kulkevat paketit ja antaa käyttäjän tutkia niitä graafisessa
käyttöliittymässä. Voit ladata järjestelmällesi sopivan asennuspaketin
[Wiresharkin kotisivulta](https://www.wireshark.org/). Verkkopakettien
kaappaaminen edellyttää Wiresharkin suorittamista järjestelmänvalvojan
oikeuksilla.

Wiresharkin käynnistyessä näet alla olevan kaltaisen näkymän. Ikkunan alaosassa
ovat järjestelmän verkkorajapinnat, joista paketteja voidaan kaapata.
Kuvakaappaus on Mac-kannettavasta, jossa on useita paikallisia liitäntöjä.
Huomionarvoisia ovat tässä esimerkissä kaiken Internet-liikenteen välittävä
langaton liitäntä **en0** sekä koneen sisäiseen viestintään käytettävä
loopback-liitäntä **lo0**.

![Wiresharkin aloitusnäkymä](/images/intro-wireshark-ifaces.png)

Kun tuplaklikataan esimerkiksi _en0_-rajapintaa, kaikki sen läpi kulkevat
paketit näytetään seuraavassa näkymässä omilla riveillään. Nopeasti huomataan,
että jo yhdessä kannettavassa tietokoneessa tapahtuu paljon viestintää:
muutamassa sekunnissa kaapataan satoja paketteja.

Tietyn protokollan toiminnan tutkimiseksi satojen tai tuhansien pakettien
joukosta on hyödyllistä rajata kiinnostava liikenne pakettisuodattimella.
Wiresharkissa tukee monipuolista notaatiota, jolla paketteja voi valita
protokollakenttien arvojen ja useita ehtoja yhdistävien loogisten operaatioiden
perusteella. Alla oleva kuvakaappaus rajaa paketit niiden UDP-lähde- tai
kohdeportin 53 perusteella suodattimella `_udp.port == 53_`. Kuvassa näkyy tulos
sen jälkeen, kun _digillä_ on tehty A-kysely nimelle _www.aalto.fi_.

![Pakettikaappausnäkymä](/images/intro-wireshark-dns.png)

Ikkunan yläosassa kukin suodatusta vastaava paketti näkyy omalla rivillään:
kuvassa ovat DNS-kysely ja DNS-vastaus. Jälkimmäinen paketti on valittu, ja sen
yksityiskohdat näkyvät ikkunan alaosassa. Vasemmalla eri protokollakerrosten
otsakkeet esitetään luettavassa muodossa linkkikerrokselta alkaen (Ethernet -->
IPv4 --> UDP --> DNS). Oikealla sama sisältö näkyy käsittelemättömänä
heksadesimaalidumppina. Wireshark näyttää siis paljolti samat asiat
DNS-paketista kuin _dig._

## Gitin käyttäminen

**[Git](https://git-scm.com/)** on nykyään käytetyin versionhallintajärjestelmä,
ja suurin osa avoimen lähdekoodin projekteista käyttää sitä. Jos työskentelet
tulevaisuudessa ohjelmistokehityksen parissa, tulet lähes varmasti käyttämään
Gitiä. Myös tällä kurssilla työstettävä ohjelmaprojekti ylläpidetään Gitissä.

Git on hajautettu järjestelmä. Palvelimella on kaikkien kehittäjien
käytettävissä oleva **repositorio**, josta kehittäjät kloonaavat kopion omaa
työtään varten. Repositorio muodostuu **commit**-tapahtumista, joissa
lähdekoodia, tekstiä tai muita repositorion tiedostoja on muutettu. Kukin commit
on looginen muutoskokonaisuus, joka voidaan synkronoida muiden kehittäjien
kanssa. Synkronointi tapahtuu **push**-toiminnolla, jolla paikalliset muutokset
lähetetään yhteiselle palvelimelle, ja **pull**-toiminnolla, jolla palvelimella
olevat uusimmat tapahtumat ladataan omaan repositorioon.

Alla oleva kuva havainnollistaa työskentelytapaa. Jukka ja
Liisa ovat samassa projektiryhmässä ja ovat luoneet yhteisen repositorion
_version.aalto.fi_-palvelimelle. Molemmat ovat kloonanneet siitä paikallisen
kopion omalle koneelleen. Git-commitit ja muut metatiedot sisältävän repositorion
lisäksi koneilla on projektin lähdetiedostojen paikalliset työkopiot. Tällä kurssilla käytössä
on myös kurssipalvelin, joka kloonaa Git-repositoriot sekä rakentaa ja suorittaa
palvelintoteutukset Docker-kontissa julkisessa osoitteessa, jotta muut opiskelijat
voivat testata niitä.

![Gitin yleiskuva](/images/intro-git.svg){: width="90%" .center-img }

### Git-repositorion perustaminen

Julkiset Git-palvelut, kuten **[GitHub](https://github.com/)** ja
**[GitLab](https://about.gitlab.com/)**, tarjoavat verkkokäyttöliittymän
repositorioiden luomiseen ja hallintaan. Git-versionhallinnan lisäksi niissä on
työkaluja esimerkiksi ongelmien raportointiin, työnprosessien hallintaan ja
jatkuvaan integraatioon. Kurssilla käytetään ensisijaisesti Aallon omaa
GitLab-pohjaista **[version.aalto.fi](https://version.aalto.fi/)**-palvelua,
johon kirjaudutaan Aalto-tunnuksilla. Siitä puuttuu (tai on rajoitetusti)
joitakin esimerkiksi GitHubin tarjoamia ominaisuuksia, kuten **jatkuvan
integraation** tuki. Jos haluat käyttää tällaisia ominaisuuksia projektissasi,
voit käyttää esimerkiksi GitHubia, mutta sovi asiasta ensin kurssihenkilökunnan
kanssa.

Kun olet kirjautunut _version.aalto.fi_-palveluun, voit luoda uuden repositorion
klikkaamalla sivun oikean yläkulman plusmerkkiä. Valitse ”Create blank project”
ja täytä projektin tiedot. Anna ensin projektille nimi. Järjestelmä ehdottaa
nimen perusteella projektille URL-osoitetta. **Aseta projekti tässä vaiheessa
yksityiseksi**, jotta vain sinä, mahdollinen projektikumppanisi ja kurssin
henkilökunta pääsevät siihen. Jätä ”Initialize with README” valituksi, jotta
uudessa repositoriossa on valmiiksi sisältöä. Klikkaa lopuksi ”Create project”.

### SSH-avainten määrittäminen

Gitiä käytetään pääasiassa komentoriviltä, vaikka kehitysympäristöt tarjoavat
graafisen käyttöliittymän tavallisimpiin toimintoihin. Koneellasi oleva
git-asiakasohjelma viestii julkisen palvelimen kanssa useimmiten
**SSH-protokollalla**. SSH:ta käytetään usein etäkomentoyhteyksiin, mutta se
toimii myös järjestelmien välisenä suojattuna viestintäkanavana.

SSH:n käyttöä varten tarvitset tunnistautumiseen avainparin. Julkinen avain
määritetään Git-palvelimelle ja yksityinen avain säilytetään omalla koneellasi.
Jos olet tehnyt tämän aiemmin, voit käyttää nykyisiä avaimiasi. Jos asia on
uusi, Aallon Git-palvelussa on
**[ohjeet](https://version.aalto.fi/gitlab/help/user/ssh.md)** avainten
määrittämiseen.

### Repositorion kloonaaminen

Kun avaimet on määritetty, voit kloonata äsken luomasi repositorion omalle koneellesi:

    git clone git@version.aalto.fi:psarolah/my-repo.git

Korvaa esimerkin osoite oman repositoriosi osoitteella. Löydät osoitteen
repositorion webliittymän pääsivun sinisen ”Code”-painikkeen alta. Valitse
osoitteen **SSH**-versio.

README-tiedostosta pitäisi nyt olla paikallinen kopio koneellasi, ja voit alkaa
lisätä projektiin muita tiedostoja.

### Perustoiminnot

Graafiset kehitysympäristöt, kuten **VS Code**, tukevat Gitin perustoimintoja.
Etsi vasemmasta sivupalkista haarautuvaa puuta muistuttava kuvake, joka on
tavallisesti kolmantena ylhäältä, mikäli haluat kokeilla. Seuraavassa käydään
läpi kuitenkin tärkeimmät komentorivikomennot.

Kun olet kehittänyt ohjelmaa jonkin aikaa ja päässyt sopivaan vaiheeseen
kehittämäsi ominaisuuden kanssa, muutoksista tehdään **commit**. Älä committaa
koodia, jonka tiedät olevan toimimatonta tai joka ei mene kääntäjästä läpi.
Tarkista ennen committia, että koodi toimii ainakin niin hyvin, että muut
kehittäjät voivat jatkaa työskentelyä siitä.

Merkitse committiin liittyvät tiedostot komennolla

    git add <file1> <file2>...

Myös uudet tiedostot lisätään näin. Jos olet luonut uusia alihakemistoja,
merkitse niiden sisällä olevat tiedostot. Mikäli toimit VS Coden
käyttöliittymässä, Git näkymässä + - ikonin painallus tiedoston kohdalla ajaa
saman asian (se siirtyy "Staged changes" - osioon)

Tee varsinainen commit:

    git commit

Komento avaa tekstieditorin komentoriville, johon kirjoitetaan lyhyt kuvaus
commitista (tai voit painaa "Commit" - nappia VS Codessa). Tämä luo paikalliseen
Git-repositorioon uuden commitin, jota ei vielä ole synkronoitu etärepositorion
kanssa. Muutokset lähetetään palvelimelle komennolla

    git push

Commitit päivittyvät nyt palvelimelle, ja muutosten pitäisi nyt näkyä myös
_version.aalto.fi_-palvelun webkäyttöliittymässä.

Jos työskentelet toisen henkilön kanssa, palvelimen mahdolliset muutokset
kannattaa synkronoida paikalliseen repositorioon komennolla

    git pull

Tästä on hyötyä myös yksin tehtävässä projektissa, jos työskentelet usealla
koneella ja haluat pitää tekemäsi muutokset synkronoituina koneiden välillä.

<div class="assignment-frame" markdown="1">

## Tehtävä 1

Ensimmäisessä tehtävässä perustetaan tulevissa tehtävissä käytettävä
Git-repositorio ja tutustutaan tässä moduulissa käsiteltyihin työkaluihin.

Perusta ensin kurssityöllesi **yksityinen** Git-repositorio. Kun olet luonut
repositorion ja testannut sen toiminnan, ilmoita sen URL-osoite
[MyCourses-kyselyssä](https://mycourses.aalto.fi/mod/questionnaire/view.php?id=1528203).
Anna myös kurssihenkilökunnalle repositorion lukuoikeus.

Avaa Wireshark ja aloita pakettien kaappaaminen oikeasta verkkorajapinnasta. Eri
laitteilla rajapintojen nimet voivat vaihdella, mutta Wireshark esittää eri
laitteilla esiintyvän liikenteen pienellä graafille, mistä on hyötyä oikean
löytymisessä.

Valitse pari tunnettua organisaatiota, mutta älä Aalto-yliopistoa, esimerkiksi
yritys tai jokin muu yliopisto, ja selvitä mitä DNS-nimeä ne käyttävät. Tee
seuraavat vaiheet ja raportoi tulokset MyCoursesiin palautettavassa
tehtäväraportissa.

1. Tarkista valitsemiesi organisaatioiden IPv4-osoitteet **digillä** eli tee
   A-kysely ja sisällytä kaikki saamasi osoitteet vastaukseen. Kokeile myös,
   löytyykö samalle nimelle IPv6-osoite, ja raportoi se. Onko vastauksessa
   CNAME-alias, joka ohjaa johonkin toiseen nimeen?

2. Kuinka monta edellä tehdyn toiminnon tuottamaa DNS-pakettia näet
   Wiresharkissa? UDP-portin 53 suodatin helpottaa pakettien löytämistä.

3. Tee **netcatilla** HTTP-pyyntö valitsemiesi kohteiden TCP-porttiin 80.
   Millaisen vastauksen saat? Mikä on ensimmäisen rivin HTTP-vastauskoodi ja
   mitä se tarkoittaa? Voit etsiä tiedon Internetistä.

4. Tarkista Wiresharkista, kuinka monta TCP-pakettia tämän toiminnon seurauksena
   siirrettiin edestakaisin TCP-porttiin 80. Kannattaa käyttää Wiresharkissa
   suodatinta TCP-portille 80. Selitä omin sanoin, miten yhteys etenee ja
   millaisia TCP-paketteja näet sen aikana. Mitä lippuja ja TCP-valitsimia
   yhteydenmuodostuksen kättelyssä käytetään? Mitä ne tarkoittavat? Voit
   selvittää asiaa verkosta.

Vastaa lopuksi seuraaviin kysymyksiin:

- Kuinka paljon aikaa käytit tehtävään?
- Mikä tehtävässä oli helppoa tai vaikeaa?

</div>
