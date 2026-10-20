---
title: Tietoturva
lang: fi-FI
translation_key: security
---

Internetin protokollat suunniteltiin alun perin suojaamattomaan tiedonsiirtoon,
jossa kuka tahansa tiedonsiirtoreitille pääsevä pystyi lukemaan viestit. Ilman
erillistä salausprotokollaa IP:n päällä kulkeva verkkoliikenne on
oletusarvoisesti salaamatonta, ja sitä voi lukea esimerkiksi Wiresharkin
kaltaisilla työkaluilla. Kuka tahansa jolla on pääsy yhteyspolulle voi siis
lukea ja muokata viestejä. Tämä voi onnistua esimerkiksi WiFi-reitittimen kautta
WiFi-reitittimeen.

Myös verkkotunnuksia IP-osoitteiksi muuntava nimipalvelujärjestelmä (DNS)
perustuu salaamattomaan tiedonsiirtoon, sen lisäksi että tietueita replikoidaan
ja tallennetaan välimuisteihin, mikä mahdollistaa väärennetyn IP-osoitetiedon.
Yhteydespolulla oleva osapuoli voi myös muuttaa IP-osoitteita pakettien
otsakkeissa. Suojaamattomassa tiedonsiirrossa emme siksi voi olla varmoja, että
palvelin todella on se, joksi sitä luulemme. Tarvitsemme ratkaisun, jolla
palvelimen identiteetti voidaan todentaa luotetun kolmannen osapuolen avulla.

Internetin alkuperäisen käyttöönoton jälkeen on kehitetty erilaisia
tietoturvaratkaisuja. Yksi ratkaisu on **IPsec ([RFC
4301](https://datatracker.ietf.org/doc/html/rfc4301))**, joka verkkokerroksella
suojaa IP-liikenteen eheyden ja salaa sen. IPseciä käytetään usein
organisaatioiden virtuaalisissa yksityisverkoissa (VPN). IPsec edellyttää
yleensä käyttöjärjestelmän tukea, joten sen käyttöönotto vaatii vain vähän
vaivaa.

Toinen, nykyään luultavasti yleisin ratkaisu on **Transport Layer Security
(TLS, [RFC 9846](https://datatracker.ietf.org/doc/html/rfc9846))** -protokolla,
jota käsittelemme seuraavaksi. Se toimii kuljetuskerroksen, tyypillisesti TCP:n,
päällä, ja sitä käytetään nykyään esimerkiksi käytännössä kaikessa
HTTP-liikenteessä. TLS on helpompi ottaa käyttöön, koska se voidaan liittää
ohjelmistoon käyttäjätilassa toimivana kirjastona eikä se tarvitse erillistä
käyttöjärjestelmätukea. TLS ei kuitenkaan suojaa IP- ja TCP-otsakkeita, ellei
liikennettä tunneloida TLS-suojatun yhteyden sisään.

<div class="objectives-frame" markdown="1">

**Tämän moduulin tavoitteet:**

- **Ymmärrät Transport Layer Security (TLS) -protokollaan perustuvan
  tietoturva-arkkitehtuurin.**

- **Opit toteuttamaan TLS-suojattua tiedonsiirtoa käyttäviä sovelluksia**
  Rust-ohjelmointikielellä.

- **Osaat käsitellä TLS-varmenteita OpenSSL-työkalulla**, esimerkiksi tarkistaa
  varmenteen tiedot ja kelpoisuuden, sekä luoda varmenteita.

- **Osaat käyttää JSON Web Tokeneita käyttöoikeuksien
  valtuuttamiseen.**

</div>

## Transport Layer Security

**Transport Layer Security (TLS)** pyrkii tarjoamaan kolmenlaista tietoturvaa:

- **Luottamuksellisuus**: Liikenne salataan, jotta ulkopuoliset eivät voi lukea sitä.
- **Eheys**: Havaitaan, jos ulkopuolinen on muuttanut tietoja.
- **Todentaminen**: Viestinnän osapuolen identiteetti varmistetaan luotetun
  kolmannen osapuolen avulla. Yleensä todennetaan palvelin, mutta myös asiakas
  voidaan todentaa.

TLS voidaan nähdä erillisenä kerroksena TCP:n päällä ja sovellusten alla, joten
se sopii hyvin perinteiseen kerroksittaiseen protokollamalliin. Alla oleva kuva
on verkkokirjasta ”[High Performance Browser
Networking](https://hpbn.co/transport-layer-security-tls/)”.

![TLS protokollapinossa](https://hpbn.co/assets/diagrams/9873c7441be06e0b53a006aac442696c.svg){: width="120%" .center-img }

### Varmenteet

TLS-protokollan olennainen osa on **varmenne**, jolla palvelimen aitous
todennetaan. Varmenteen luo ja allekirjoittaa luotettu **varmentaja
(certificate authority, CA)**, ja palvelin lähettää sen asiakkaalle
TLS-yhteyden muodostamisen yhteydessä. Varmenne sisältää muun muassa
seuraavat tiedot:

- **Yleinen nimi**, jota yleensä verrataan palvelimen DNS-nimeen.
- **Palvelimen julkinen avain**, jota käytetään palvelimen autentikoitumiseen.
- **Voimassaoloaika**, jolloin varmennetta voidaan käyttää.
- **Myöntäjä**, eli varmenteen luoneen varmentajan nimi.
- **Digitaalinen allekirjoitus**, jonka varmentaja on luonut ja joka voidaan
  tarkistaa varmentajan julkisella avaimella.

Asiakas tarkistaa varmenteen, muunmuassa että yleinen nimi vastaa palvelimen
nimeä, ja että varmenne on yhä voimassa. Asiakkaan on myös luotettava varmenteen
myöntäjään. Paikalliseen järjestelmään on yleensä määritetty luotetut
**juurivarmentajat**. Juurivarmentajat voivat välittää varmentamisvastuuta
välivarmentajille, jotka luovat varsinaisen varmenteen ja toimittavat sen
palvelimen omistajalle. Näin muodostuu varmenneketju, joka alkaa palvelimen
varmenteesta ja jonka tulee päättyä johonkin luotetuista juurivarmentajista.

Joissakin sisäisissä käyttökohteissa, kuten yritysten intraneteissä tai tämän
kurssin palvelinkonteissa, varmenteen hankkiminen yleisesti luotetulta
julkiselta varmentajalta ei välttämättä ole käytännöllistä. Tällaisiin
ympäristöihin voidaan kuitenkin yleensä määrittää omia paikallisia
juurivarmentajia, mikä helpottaa varmenteiden käyttöönottoa. Näin toimimme myös
tällä kurssilla (käytännön toteutukseen palataan hieman myöhemmin).

### TLS-kättelyprotokolla

Kun halutaan käyttää TLS-suojattua tiedonsiirtoa, asiakas avaa TCP-yhteyden
tavalliseen tapaan. Yhteyden muodostamisen jälkeen asiakas aloittaa TLS-kättelyn,
joka etenee seuraavasti:

- Asiakas lähettää **ClientHello**-viestin, jossa se ilmoittaa esimerkiksi
  tukemansa TLS-versiot ja salausalgoritmit.
- Palvelin vastaa **ServerHello**-viestillä, jossa se valitsee yhden ehdotetuista
  salausalgoritmeista sekä toimittaa varmenteen ja varsinaisessa tiedonsiirrossa
  käytettävän symmetrisen avaimen muodostamiseen tarvittavat tiedot.
- Asiakas tarkistaa varmenteen, ja molemmat osapuolet laskevat jaetun
  salaisuuden, josta muodostetaan varsinaisessa tiedonsiirrossa käytettävä
  symmetrinen istuntoavain.
- Molemmat osapuolet vahvistavat kättelyn **Finished**-viestillä, minkä jälkeen
  tiedonsiirto jatkuu istuntoavaimella suojattuna.

Symmetrisen istuntoavaimen käytöstä on etua: sen käyttö on tehokkaampaa, ja
uuden avaimen käyttäminen jokaisessa istunnossa parantaa turvallisuutta.

Edellä kuvattua kättelyä havainnollistetaan alla olevassa kuvassa, joka on
alun perin Cloudflaren blogikirjoituksesta ”[What happens in a TLS
handshake](https://www.cloudflare.com/learning/ssl/what-happens-in-a-tls-handshake/)”.
Kuvasta nähdään, että vaikka TLS on nykyään yleinen tapa suojata verkkoliikennettä,
sen haittapuolena on yhteyden muodostamiseen kuluva lisäviive ennen varsinaisen
tiedonsiirron alkamista. Viiveen suhteellinen merkitys korostuu, jos yhteyden
aikana siirretään vain vähän dataa.

![TLS-protokollan kättely](https://images.ctfassets.net/slt3lc6tev37/5aYOr5erfyNBq20X5djTco/3c859532c91f25d961b2884bf521c1eb/tls-ssl-handshake.png){: width="90%" .center-img }

## TLS Rustissa

Suosittu kirjasto TLS:n käyttämiseen Rust-sovelluksissa on
**[rustls](https://crates.io/crates/rustls)**. Se tarjoaa rajapinnan
TLS:n käyttöön TCP:n päällä.

### Asiakkaan toiminta

Esimerkkihakemiston
**[tls-client](https://github.com/PasiSa/pronets/blob/main/examples/tls-client/src/main.rs)**-hakemistossa
on muokattu versio aiemmin esitetystä yksinkertaisesta asiakasohjelma. Voit
jälleen kloonata repositorion ja kokeilla esimerkkiä omalla koneellasi.

Aiempien esimerkkien tapaan ohjelma ottaa muutamia komentoriviargumentteja. Nyt
mukana on myös valinnainen argumentti juurivarmennetiedoston määrittämiseen. Jos
kokeilet ohjelmaa julkisella palvelimella (esimerkissä käytetään palvelinta
`www.aalto.fi`), tätä ei tarvita, sillä `webpki-roots`-kirjasto sisältää
yleisesti hyväksytyt juurivarmenteet. Paikallisessa testauksessa meidän on
kuitenkin määritettävä oma juurivarmenteemme, jolla voimme tarkistaa palvelimen
lähettämän varmenteen. Esimerkkihakemisto sisältää tiedoston `cert/ca.crt`, jota
on käytetty **tls-server**-esimerkin mukana toimitettujen varmenteiden
allekirjoittamiseen, ja jota voi käyttää testaamiseen palvelinesimerkin kanssa.

Aluksi, [riviltä
34](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L34)
alkaen, erotetaan palvelimen nimi komentoriviargumentista. Nimeä verrataan
saapuvan varmenteen yleiseen nimeen, ja niiden on täsmättävä. _rustls_ tarjoaa
tätä varten oman `ServerName`-tyypin.

[Riviltä
44](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L44)
alkaen suoritus haarautuu sen mukaan, käytämmekö käyttäjän antamaa juurivarmennetta
vai yleisesti hyväksyttyjä juurivarmenteita. Käytettävissä olevat juurivarmenteet
tallennetaan `roots`-muuttujaan, jonka tyyppi on `RootCertStore`.

Näiden perusteella asetetaan TLS-asiakkaan asetukset ([rivi
62](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L62))
ja yhdistetään TCP-asiakaspistoke tavalliseen tapaan. Sen jälkeen luodaan
`ClientConnection`-olio annettuja astuksia käyttäen, joka myös käynnistää
TLS-kättelyn. Jos kättely onnistuu ja palvelimen varmenne on kelvollinen,
TLS-yhteys liitetään TCP-pistokkeeseen **rustls**-kirjaston `StreamOwned`-tyypin
avulla ja tallennetaan `socket`-muuttujaan. Tämän jälkeen pistoketta voidaan
käyttää luku- ja kirjoitusoperaatioita normaalisti, ja kaikki liikenne salataan
TLS:llä.

Lopuksi asiakas ilmoittaa yhteyden sulkemisesta `send_close_notify`-funktiolla
[rivillä
78](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-client/src/main.rs#L78).
Tämän avulla voidaan kertoa että yhteyden sulkeminen tapahtuu suunnitellusti,
eikä sitä ole keskeytetty jostain muusta syystä.

### Palvelimen toiminta

TLS-palvelinesimerkki löytyy kurssin git-repositorion
**[tls-server](https://github.com/PasiSa/pronets/blob/main/examples/tls-server/src/main.rs)**-hakemistosta.
Se muistuttaa aiempaa Tokiota käyttävää
[asynkronista palvelinesimerkkiä](https://github.com/PasiSa/pronets/blob/main/examples/async-server/src/main.rs),
mutta käyttää nyt TLS:ää. TLS:n käyttämiseen Tokion kanssa on erillinen
**[tokio-rustls](https://crates.io/crates/tokio-rustls)**-kirjasto, jota
esimerkissä käytetään.

Kuunneltavan IP-osoitteen ja portin lisäksi palvelin ottaa nyt lisäksi
komentoriviparametreina PEM-koodatut tiedostot, jotka sisältävät **varmenteen**
ja varmenteen julkista avainta vastaavan **yksityisen avaimen**.
_rustls_-kirjasto lukee nämä tiedostot ja määrittää palvelimen asetukset niiden
perusteella ([rivi
46](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-server/src/main.rs#L46)).
Esimerkkiä testatessa voidaan käyttää repositorion mukana tulevaa
palvelinvarmennetta (_cert/server.crt_) ja yksityistä avainta
(_cert/server.key_). Huomaa, että **todellisessa käytössä näitä tiedostoja ei
koskaan lisättäisi git-repositorioon, ja erityisesti yksityinen avain on
pidettävä salassa!** Myöskään järjestelmän muut käyttäjät eivät saa pystyä
lukemaan yksityistä avainta. Varmenne on luotu _tls-client_-hakemistossa olevan
_ca.crt_-varmenteen avulla, jota voidaan käyttää varmenteen tarkistamiseen.

Asiakaspuolen **ClientConnection**-tyypin tavoin **TlsAcceptor** muuntaa
TCP-yhteyden TLS-virraksi TLS-asetusten avulla ([rivi
51](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-server/src/main.rs#L51)).
Se kääritään atomista viitelaskentaa käyttävään **Arc**-viitteeseen, koska jokaisen
Tokio-tehtävän on päästävä käyttämään sitä.

TCP-palvelinpistoke sidotaan osoitteeseen tavalliseen tapaan, ja uusi yhteys
hyväksytään ensin normaalisti TCP-kuuntelijan `accept()`-funktiolla. Sen jälkeen
yhteys hyväksytään erikseen **TlsAcceptor**-instanssin `accept()`-funktiolla ([rivi
69](https://github.com/PasiSa/pronets/blob/dbf3770c7a4858e826b7782a2c7e496c57863c88/examples/tls-server/src/main.rs#L69)),
joka viimeistelee TLS-neuvottelun. Tämän jälkeen pistokkeesta voidaan lukea ja
siihen voidaan kirjoittaa tavalliseen tapaan.

## Varmenteiden käsittely

**OpenSSL** on yleinen työkalu erilaisiin TLS:ään liittyviin tehtäviin, kuten
varmenteiden tarkasteluun ja luomiseen (SSL eli Secure Sockets Layer on nykyisen
TLS-standardin edeltäjä). Se on usein valmiiksi asennettuna järjestelmään. Jos
näin ei ole, esimerkiksi Ubuntu Linuxissa sen voi asentaa tavalliseen tapaan
_apt_-työkalulla: `sudo apt install openssl`

Varmennetta voi tarkastella komentorivillä seuraavasti (tässä varmenne on
tiedostossa _server.crt_):

    openssl x509 -in server.crt -text -noout

Tämä varmenne ei ole kovin hyvä: esimerkiksi sen voimassaoloaika on hyvin pitkä.
Sitä käytetään kuitenkin vain tällä kurssilla varmenteiden käsittelyn
harjoitteluun.

_openssl_-työkalulla voi myös tarkistaa tietyssä verkko-osoitteessa käytettävän
varmenteen ja näyttää sen tiedot. Esimerkiksi _www.aalto.fi_-palvelimen
varmennetta voi tarkastella näin:

    openssl s_client -connect www.aalto.fi:443

### Juurivarmenteen luominen

Jotta voidaan luoda varmenne omalle **juurivarmentajalle**, juurivarmenteen
ylläpitäjän (eli kun tehdään paikallisia testejä omalla toteutuksella, sinun)
pitää luoda ensin **yksityinen avaimen**:

    openssl genpkey -algorithm RSA -out rootCA.key -pkeyopt rsa_keygen_bits:4096

Yksityinen avain on pidettävä salassa ja säilytettävä turvallisessa paikassa.
Kuka tahansa, jolla on pääsy avaimeen, voi luoda varmenteita juurivarmentajan
nimissä.

Seuraavaksi luodaan OpenSSL-asetustiedoston, joka määrittää juurivarmenteen
tiedot:

    [ req ]
    distinguished_name = dn
    x509_extensions = v3_ca
    prompt = no

    [ dn ]
    CN = My Example Root CA
    O = Example Organization
    C = FI

    [ v3_ca ]
    basicConstraints = critical,CA:true
    keyUsage = critical,keyCertSign,cRLSign
    subjectKeyIdentifier = hash
    authorityKeyIdentifier = keyid:always

Tämän jälkeen luodaan asetusten perusteella itse allekirjoitettu juurivarmenne
tiedostoon _rootCA.crt_. Tässä yksityinen avain on tiedostossa _rootCA.key_ ja
yllä olevan kaltaiset asetukset tiedostossa _root.cnf_.

    openssl req \
        -new \
        -x509 \
        -days 3650 \
        -key rootCA.key \
        -out rootCA.crt \
        -config root.cnf

### Palvelimen varmenteen luominen

Näiden vaiheiden jälkeen juurivarmentaja voi luoda varsinainen
palvelinvarmenteen. Vaiheet ovat samankaltaiset, mutta tässä on hyödyllistä
erottaa eri osapuolten tehtävät juurivarmetajan ja palvelimen ylläpitäjän
välillä. **Palvelimen ylläpitäjä** luo yksityisen avaimen:

    openssl genpkey \
        -algorithm RSA \
        -out server.key \
        -pkeyopt rsa_keygen_bits:2048

Yksityisen avaimen sisältävän tiedoston _server.key_ käyttöoikeudet tulee asettaa
niin, että vain palvelimen ylläpitäjä voi lukea sitä.

Palvelimen ylläpitäjä luo sitten varmenteen asetustiedoston, esimerkiksi:

    [ req ]
    distinguished_name = dn
    req_extensions = server_extensions
    prompt = no

    [ dn ]
    CN = my.server.fi
    O = Example Organization
    C = FI

    [ server_extensions ]
    basicConstraints = critical,CA:false
    keyUsage = critical,digitalSignature,keyEncipherment
    extendedKeyUsage = serverAuth
    subjectAltName = @alternative_names

    [ alternative_names ]
    DNS.1 = my.server.fi
    DNS.2 = localhost
    IP.1 = 192.168.1.20
    IP.2 = 127.0.0.1
    IP.3 = ::1

Palvelimen DNS-nimen ja IP-osoitteen tulee vastata yleistä nimeä (`CN`-kenttä)
tai vaihtoehtoisten nimien tai IP osoitteiden kenttiä (`DNS.X` tai `IP.X`).
Kuten huomataan, sama sertifikaatti voi tukea useita nimiä tai IP-osoitteita,
esimerkiksi eri verkkorajapintoja tai näkyvyysalueita varten. Paikallisessa
testauksessa voi käyttää nimeä `localhost`.

Seuraavaksi _palvelimen ylläpitäjä_ luo **varmenteen allekirjoituspyynnön**
asetustiedoston (tässä _server.cnf_) ja yksityisen avaimen avulla. Tässä pyyntö
tallennetaan tiedostoon _server.csr_:

    openssl req \
        -new \
        -key server.key \
        -out server.csr \
        -config server.cnf

Syntynyt _server.csr_-tiedosto sisältää julkisen avaimen ja asetustiedoston
tiedot. Se annetaan _varmentajalle_ allekirjoitettavaksi. Pyyntötiedosto on
allekirjoitettu palvelimen ylläpitäjän yksityisellä avaimella, jotta varmentaja
voi tarkistaa tiedoston sisältämän julkisen avaimen aitouden.

Tämän jälkeen _varmentaja_ luo varsinaisen varmenteen (tässä _server.crt_), joka
on voimassa 365 päivää. Varmentaja allekirjoittaa _palvelimen ylläpitäjän_
antamat varmennetiedot ja toimittaa varmenteen _palvelimen ylläpitäjälle_.

    openssl x509 \
        -req \
        -in server.csr \
        -CA rootCA.crt \
        -CAkey rootCA.key \
        -CAcreateserial \
        -out server.crt \
        -days 365 \
        -sha256 \
        -extfile server.cnf \
        -extensions server_extensions

Kun luomme varmenteita omaan käyttöömme, teemme luonnollisesti kaikki edellä
kuvatut vaiheet itse.

## Valtuutus ja JSON Web Tokenit

Käyttäjien ja asiakkaiden **todentamiseen** ja **valtuuttamiseen** on erilaisia
menetelmiä. Esimerkiksi TLS-varmenteilla voidaan todentaa myös asiakkaita samaan
tapaan kuin edellä todennettiin palvelimia. Kun molemmat osapuolet todentavat
toisensa varmenteiden avulla, puhutaan **molemminpuolisesta TLS:stä (mTLS)**.

Seuraavaksi tarkastelemme, miten asiakkaiden pyyntöjä voidaan valtuuttaa
verkkorajapinnoissa yleisesti käytettyjen **JSON Web Tokenien (JWT)** avulla.
HTTP-asiakkaat lähettävät JWT:n yleensä `Authorization`-otsakkeessa, jonka arvo
on muotoa `Bearer JWT-TOKEN`. Tästä moduulista alkaen kaikkien konttien
käynnistyspyyntöjen on sisällettävä kelvollinen JWT ja käytettävä TLS-suojattua
yhteyttä.

### Todentaminen ja valtuutus

**Todentaminen** vastaa kysymykseen _”Kuka olet?”_. Järjestelmän on pystyttävä
selvittämään käyttäjän tai asiakkaan henkilöllisyys. Tämä voi tapahtua
käyttäjätunnuksella ja salasanalla kirjautumalla, varmenteiden avulla tai
käyttämällä ulkoista tunnistuspalvelua, esimerkiksi pankkitunnuksia.

Kun käyttäjän henkilöllisyys on tiedossa, on päätettävä, _mitä oikeuksia
käyttäjällä on ja mitä resursseja hän voi käyttää_. Tätä kutsutaan
**valtuutukseksi**. Hajautetussa palvelussa yksi tapa välittää nämä tiedot
palvelun eri osille on käyttää valtuuttajan allekirjoittamaa JSON Web Tokenia.
JWT ei siis ole todentamisprotokolla, vaan formaatti valtuutustietojen
välittämiseen siten, että vastaanottaja voi tulkita ne luotettavasti ja
tilattomasti.

Kurssin palvelin ei toteuta todentamista, mutta tästä moduulista alkaen se
edellyttää JWT:tä osana Docker-kontin käynnistyspyyntöjä. Kurssihenkilökunta
toimittaa kullekin projektille MyCoursesin kautta JWT:n, jonka henkilökunta on
allekirjoittanut. Luotamme todentamiseen normaalia MyCoursesin käyttämää Aallon
kirjautumisjärjestelmää.

### JWT:n rakenne

**JSON Web Token** (JWT, määritelty [RFC
7519:ssä](https://datatracker.ietf.org/doc/html/rfc7519)) on JSON-muotoinen
tietorakenne, joka koodataan _Base64URL_-koodauksella (määritelty [RFC
4648:ssa](https://datatracker.ietf.org/doc/html/rfc4648)). Koodaus muuntaa
JSON-rakenteen tiiviiksi 64 merkin merkistöä käyttäväksi merkkijonoksi, jota
protokollat kuten HTTP voivat käsitellä. JWT voi olla tyypiltään joko **JSON Web
Signature (JWS)** tai **JSON Web Encryption (JWE)**. Seuraavassa käsittelemme
allekirjoitukseen käytettävää JWS:ää, joka tarkemmin [RFC
7515:ssä](https://datatracker.ietf.org/doc/html/rfc7515).

JWT:n hyödyllinen ominaisuus on, ettei se välttämättä edellytä tilan
ylläpitämistä palvelimella: kaikki valtuutukseen tarvittavat tiedot sisältyvät
itse tokeniin. Token koostuu kolmesta osasta:

- **Otsake** (Header, myös JSON Object Signing and Encryption eli JOSE Header)
  sisältää tiedon datan allekirjoittamiseen käytetystä algoritmista.
- **Hyötykuorma** (Payload) sisältää tokenin esittämät attribuutit eli väitteet
  (claims).
- **Allekirjoitus** (Signature) sisältää tokenin myöntäjän tekemän
  allekirjoituksen, joka on luotu otsakkeessa ilmoitetulla algoritmilla.

Tokenin tyypillinen otsake on seuraavalainen:

```json
{
  "typ": "JWT",
  "alg": "HS256"
}
```

Tämä kertoo, että kyseessä on JSON Web Token ja että token allekirjoitetaan
jaettuun salaisuuteen perustuvalla **HS256**-algoritmilla. Hyötykuorma sisältää
esimerkiksi seuraavanlaisia väitteitä:

```json
{
  "sub": "alice",
  "role": "student",
  "exp": 1791910800
}
```

Base64URL-koodatut tokenit alkavat usein merkeillä `eyJ0...`, koska otsakeosa on
usein samanlainen, ja alkaa aaltosuluilla. On hyvä ymmärtää, että vaikka token
näyttää ensi silmäyksellä vaikealukuiselta, sitä ei ole salattu, ja
Base64URL-koodaus voidaan purkaa takaisin JSON-muotoon. Tämä on melko
suoraviivainen ohjelmointitehtävä [RFC
4648:n](https://datatracker.ietf.org/doc/html/rfc4648) ohjeiden avulla. Verkossa
on myös työkaluja tokenien [koodaamiseen](https://www.base64encode.org/) ja
[purkamiseen](https://www.base64decode.org/). Tokenin lopussa olevan
allekirjoituksen tarkoituksena on kuitenkin varmistaa, ettei ulkopuolinen voi
muuttaa väitteiden sisältöä ilman, että muutos havaitaan tokenia
varmennettaessa.

### Väitteet (claims)

JSON-väitteitä on kahdenlaisia: rekisteröidyt väitteiden nimet määritellään
JWT-spesifikaatiossa ([RFC 7519, luku
4](https://datatracker.ietf.org/doc/html/rfc7519#section-4)), ja ne ovat yleisiä
attribuutteja, joita voidaan käyttää eri palveluissa ja sovelluksissa.
Rekisteröityjen väitteiden lisäksi voi olla sovelluskohtaisia väitteitä, jotka
sovilluksen kehittäjä voi vapaasti päättää sovelluksen tarpeiden perusteella.

Spesifikaatiossa määritellään yhteensä seitsemän rekisteröityä väitteen nimeä.
Tässä on muutamia yleisiä:

- **`iss` (issuer, myöntäjä)** kertoo, kuka myönsi (tai loi) JWT:n. Myöntäjän
  nimen muotoa ei ole määritelty, vaan se on sovelluskohtainen. Se voi perustua
  esimerkiksi käyttäjän todentaneen identiteettipalvelun verkkotunnukseen.
- **`sub` (subject, kohde)** kertoo JWT:n kohteen. Se voi olla esimerkiksi
  todennetun käyttäjän käyttäjänimi (tai valtuutetun Docker-kontin nimi, kuten
  kurssipalvelimemme tapauksessa).
- **`exp` (expiration time, vanhenemisaika)** kertoo ajan, jonka jälkeen JWT ei
  ole enää voimassa eikä sitä tule hyväksyä. Vanhenemisaika ilmoitetaan
  sekunteina vuoden 1970 alusta. Tämä on standardeissa ja tietojärjestelmissä
  yleinen tapa esittää aikoja kokonaislukuina. Tokeneille on yleensä hyvä
  määrittää voimassaoloaika ja tarjota tapa uusia token todennetusti ennen sen
  vanhenemista.
- **`iat` (issued at, myöntämisaika)** kertoo, milloin token luotiin. Aika
  esitetään samassa muodossa kuin vanhenemisaika (sekunteina vuoden 1970
  alusta).

### Allekirjoittaminen ja varmentaminen

JWT:iden allekirjoittamiseen on kaksi mallia. **Symmetrisessä** mallissa tokenin
myöntäjä ja sen varmentava palvelin käyttävät **yhteistä salaisuutta**. Tässä
tapauksessa käytetään yleisesti **HS256**-algoritmia. Yhteinen salaisuuden
suositellaan olevan vähintään 32 tavun mittainen satunnaisesti generoitu arvo,
joka jaetaan myöntäjän ja tokenia käyttävien palvelininstanssien kesken.
OpenSSL:ää voidaan käyttää satunnaisten arvojen generoimiseen, esimerkiksi
seuraavanlaisesti (32-tavun mittaiselle heksadesimaaliluvulle).

    openssl rand -hex 32

Vaihtoehtona on **epäsymmetrinen** malli, jossa käytetään **yksityistä avainta ja
julkista avainta**. Tällöin vain tokenin myöntäjällä on yksityinen avain, ja
kaikki tokenin avulla pyyntöjä varmentavat palvelimet käyttävät julkista avainta.
Tässä tapauksessa käytetään yleisesti **RS256**-algoritmia.

Symmetrisen mallin etuna on yksinkertaisuus, mutta yhteisen salaisuuden jakaminen
tarkoittaa, että kuka tahansa sen haltija voi luoda tokeneita. Siksi kaikkien
osapuolten on luotettava toisiinsa. Epäsymmetristä mallia suositaankin usein,
jos useiden eri palvelinten on varmennettava tokeneita.

Kun palvelin on varmentanut tokenin allekirjoituksen, se tutkii tokenin väitteet
selvittääkseen, mitä asiakas saa tehdä. Se tarkistaa esimerkiksi, että myöntäjä
`iss` on tunnettu ja että token on yhä voimassa vanhenemisajan (`exp`)
perusteella. Tokenissa voi olla myös muita sovelluskohtaisia väitteitä, jotka
tarkentavat sen antamia valtuuksia, kuten erilaisia käyttäjärooleja
(esimerkiksi opiskelija, opettaja tai ylläpitäjä).

### Toteutus Rustilla

[`jsonwebtoken`-kirjasto](https://docs.rs/crate/jsonwebtoken/10.3.0) osaa
sarjallistaa väitteet Base64URL-koodattuun muotoon ja allekirjoittaa ne JWT:n
luomiseksi. Alla ovat `Cargo.toml`-tiedostoon tarvittavat lisäykset kirjaston
käyttöönottamiseksi. **serde** on kirjasto rakenteisen tiedon sarjallistamiseen
esimerkiksi JSON-muotoon ja takaisin.

```toml
[dependencies]
jsonwebtoken = { version = "10", features = ["rust_crypto"] }
serde = { version = "1", features = ["derive"] }
```

Seuraava esimerkki luo HS256-algoritmilla allekirjoitetun tokenin, joka on
voimassa tunnin. Ennen ohjelman suorittamista `JWT_SECRET`-ympäristömuuttuja on
asetettava sisältämään vähintään 32 tavun mittainen satunnainen salaisuus, joka
jaetaan tokenin varmentavan palvelimen kanssa. Jos ohjelma käynnistetään cargon
avulla, tämän voi tehdä esimerkiksi seuraavasti:

    JWT_SECRET="myrandomsecret" cargo run

Salaisuuden voi vaihtoehtoisesti sisällyttää myös tiedostoon joka on suojattu
ulkopuolisilta. Tällöin esimerkkiä pitää tosin muuttaa hieman näiltä osin
lukemaan salaisuus annetusta tiedostosta.

```rust
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

// Generoi Serden Serialize-toteutus, jotta Claims voidaan koodata JSON-muotoon.
#[derive(Serialize)]
struct Claims {
    iss: String,
    sub: String,
    iat: u64,
    exp: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Yhteinen salaisuus on tallennettu JWT_SECRET-ympäristömuuttujaan.
    let secret = std::env::var("JWT_SECRET")?;
    if secret.len() < 32 {
        return Err("JWT_SECRET must contain at least 32 bytes".into());
    }

    // Lue tämän hetkinen aika iat:ta varten ja aseta voimassaoloajaksi yksi tunti.
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let claims = Claims {
        iss: "pns-course-staff".to_owned(),
        sub: "project-123".to_owned(),
        iat: now,
        exp: now + 60 * 60,
    };

    // jsonwebtoken-kirjaston encode-funktio luo varsinaisen
    // Base64URL-koodatun tokenin ja allekirjoittaa sen salaisuudella.
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    println!("{token}");
    Ok(())
}
```

Tässä `sub` yksilöi projektin ja `iss` tokenin myöntäjän. Palvelin varmentaa
allekirjoituksen samalla salaisuudella.

Seuraava palvelinpuolen esimerkki lukee vastaanotetun tokenin
komentoriviargumentista. Se varmentaa HS256-allekirjoituksen, vanhenemisajan ja
odotetun myöntäjän ennen projektin tunnisteen käyttämistä. Se käyttää samoja
riippuvuuksia ja samaa `JWT_SECRET`-arvoa kuin edellä oleva tokenin myöntäjä.
Kannattaa huomioida, että ympäristömuuttuja littyy aina paikalliseen
ympäristöön, ja se on erikseen asetettava palvelinpäässä.

```rust
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

// Debug mahdollistaa debug-muotoilun (käytetään {:?}-muotoilumääreellä).
// Clone mahdollistaa rakenteen kopioimisen.
// Deserialize antaa Serden muodostaa Claims-rakenteen tokenin JSON-sisällöstä.
#[derive(Debug, Clone, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    iat: u64,
    exp: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Salaisuus annetaan ympäristömuuttujassa kuten edellisessä esimerkissä.
    let secret = std::env::var("JWT_SECRET")?;
    if secret.len() < 32 {
        return Err("JWT_SECRET must contain at least 32 bytes".into());
    }
    // JWT annetaan komentoriviargumenttina.
    let token = std::env::args().nth(1).ok_or("missing JWT argument")?;

    // Varmennetaan token jsonwebtoken-kirjaston Validation-tyypin ja sen metodien avulla.
    // Toteutus hyväksyy vain odotetun myöntäjän HS256-tokenit, ja tarkistaa
    // exp-väitteen oletusarvoisesti.
    // Lisäksi edellytetään annettujen väitteiden olemassaolo.
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&["pns-course-staff"]);
    validation.set_required_spec_claims(&["exp", "iss", "sub", "iat"]);
    validation.leeway = 0;

    let verified = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    println!("Authenticated project: {}", verified.claims.sub);
    Ok(())
}
```

Varmennuksen `leeway`-parametri sallii vanhenemisajan tarkistuksessa sekunteina
annetun toleranssin, jolla huomioidaan myöntäjän ja palvelimen kellojen erot.
Esimerkiksi arvo 60 sallii tokenin läpäistä vanhenemisajan tarkistuksen vielä
60 sekuntia `exp`-ajan jälkeen. Esimerkissä `leeway` on nolla, joten
vanhentuneet tokenit hylätään ilman tätä toleranssia.

Verkkopalvelimen tulisi antaa asiakkaalta vastaanotettu token
`decode`-funktiolle, ja tarkistaa varmennettujen väitteiden perusteella, onko
asiakkaalla oikeus pyydettyyn toimintoon. Jos varmennus epäonnistuu, `decode`
palauttaa virheen ja palvelimen on hylättävä pyyntö.

## Tehtävä #7

Olet saanut JWT-tokenin, jota sinun tulee tästä eteenpäin käyttää konttisi
käynnistys- ja uudelleenkäynnistyspyyntöjen todentamiseen. Tarkemmat ohjeet
löytyvät MyCourses-tiedotteesta.

**Osa 1:** Pura saamasi token ja kerro raportissa, mitä väitteitä se sisältää.
Milloin token vanhenee? Onko tokenin pitkä voimassaoloaika hyvä ajatus?
Perustele mahdolliset hyödyt ja haitat. Onko token kelvollinen?

**Osa 2 (a):** Tarkista kurssipalvelimen
[pronets1.dice.aalto.fi](https://pronets1.dice.aalto.fi/) varmenne. Kuka on
myöntänyt varmenteen? Milloin varmenne vanhenee?

**Osa 2 (b):** Tarkista samat tiedot sisäisestä juurivarmenteesta, jota olemme
käyttäneet konttien varmenteiden luomiseen. Kuka on myöntänyt varmenteen ja
milloin se vanhenee?

**Osa 3:** Toteuta TLS-tuki palvelimeesi ja asiakkaaseesi. Kun testaat toteutusta
paikallisesti, voit luoda paikallisen juurivarmenteen sekä palvelinvarmenteen ja
yksityisen avaimen tämän moduulin ohjeiden mukaisesti.

Kun otat toteutuksesi käyttöön kurssipalvelimella, varmenteesi liitetään kontin
sisällä tiedostoon `/run/tls/server.crt` ja yksityinen avain tiedostoon
`/run/tls/server.key`. Huomaa, että tiedostot ovat eri projektien konteissa
erilaiset samoista nimistä huolimatta (Dockerin taikaa!). Lue ensin alla olevan
osan 4 ohjeet ennen kuin otat toteutuksesi käyttöön palvelimella.

Jotta sama toteutus tukisi paikallista testausta ja palvelimella käyttöä eri
varmenteilla, varmenne- ja avaintiedostojen polut kannattaa määrittää joko
komentoriviparametreilla tai ympäristömuuttujilla.

Kuvaile raportissasi lyhyesti, miten testasit toteutustasi paikallisesti ja miten
toteutit varmenteen ja avaimen määrittämisen.

**Osa 4:** Muokkaa ohjelmaa, jolla olet lähettänyt `run-docker`-HTTP-pyynnön,
seuraavasti:

- Muodosta kurssipalvelimeen `pronets1.dice.aalto.fi` TLS-yhteys TCP-porttiin 443
  (eli HTTPS-yhteys). Kurssipalvelimen varmenteen on myöntänyt yleisesti luotettu
  julkinen varmentaja, joten sinun ei pitäisi tarvita omia juurivarmenteita
  palvelimen varmentamiseen.

- Käytä uutta POST-päätepistettä `run-docker-secure`, joka toimii vain
  TLS-yhteydellä. HTTP-pyyntöön tulee lisätä uusi otsakekenttä, joka noudattaa
  JWT-tokenien välittämiseen yleisesti käytettyä muotoa:

  `Authorization: Bearer SAAMASI-JWT`

  Jos otsakekenttä puuttuu tai on virheellinen (esimerkiksi token ei ole
  kelvollinen), pyyntöä ei hyväksytä. JSON-attribuutit ovat muuten samat kuin
  `run-docker`-pyynnössä, mutta kontin nimen määrittävää `name`-kenttää ei enää
  sisällytetä pyyntöön (miksi?). Lisää kuitenkin uusi attribuutti

  `"secure": true`

  ilmaisemaan, että palvelimesi vaatii nyt TLS-yhteyden. Kurssipalvelimen
  käyttöliittymässä on pieni kuvake, joka kertoo tästä. Käytettävissä on
  tarvittaessa myös valinnainen `env`-attribuutti jonka voi sisällyttää
  JSON-sisältöön, ja jolla voit asettaa kontillesi ympäristömuuttujia,
  esimerkiksi:

  `"env": ["RUST_LOG=debug", "SECRET_VALUE=xyz"]`

Kerro raportissasi, mitä JSON-parametreja käytit POST-pyynnössä ja saitko
suojatun toteutuksesi toimimaan. Muodosta paikallisella koneellasi toimivalla
asiakkaalla yhteys palvelinkonttiisi ja testaa tähän mennessä toteutetut viestit
(TST, USR, MSG, MET).

**Osa 5 (valinnainen):** Jos sinulla on aikaa ja kiinnostusta tehdä jotain
lisää, toteuta uusi viesti `AUT` ("authorized user"), joka ottaa parametrikseen
JWT-tokenin. Tätä käytettäisiin aiempaa `USR`-viestiä turvallisempana tapana
rekisteröidä käyttäjiä palveluusi. Voit päättää projektisi tarpeiden
perusteella, mitä väitteitä tokenissa käytetään (todennäköisesti ainakin `sub`
käyttäjänimelle) ja miten tokenit jaetaan. Sinulla voisi esimerkiksi olla
yksinkertainen kirjautumispalvelu, joka luo tokenit kelvollisen käyttäjänimen ja
salasanan perusteella. Tätä osaa ei vaadita tehtävän täysiin pisteisiin. **Älä
sisällytä tokenien luomiseen ja varmentamiseen käytettävää salaisuutta
git-repositorioosi!** Edellä kuvattu `env`-attribuutti sopii hyvin tähän
tarkoitukseen.

Jos toteutit osan 5, kerro raportissa, mitä väitteitä käytät toteutuksessasi.

Sisällytä raporttiin aiempaan tapaan myös seuraavat tiedot:

- Kuinka paljon aikaa käytit tehtävään?
- Mikä tehtävässä oli helppoa tai vaikeaa?
- Mitä työkaluja tai muita tietolähteitä käytit? Jos käytit tekoälyavustajia,
  kerro erityisesti, miten käytit niitä ja oliko niistä hyötyä.
