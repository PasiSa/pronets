---
title: Projektin tavoitteet ja arviointiperusteet
lang: fi-FI
translation_key: project
---

Kurssin aikana kehitetään viikkotehtävien ohella pieni verkkosovellusprojekti,
joka koostuu tehtävissä kehitetyistä osista. Projektista saa 0–6 pistettä,
jotka lisätään tehtäväpisteisiin (7 tehtäväkierrosta, yhteensä enintään 21
pistettä).

Projektin tavoitteet ja arviointiperusteet jakautuvat seuraaviin osa-alueisiin.
Otsikoissa on ilmoitettu niiden suhteelliset painot arvioinnissa.

## Dokumentaatio (2)

Dokumentti laaditaan mieluiten git-repositorioon **Markdown-tiedostona**,
esimerkiksi täydentämällä aiemmin aloitettua `design.md`-tiedostoa.
Dokumentin tulee käsitellä seuraavat asiat:

- Projektin rajaus ja toteutetut ominaisuudet sekä kuvaus keskeisestä
  toiminnallisuudesta.
- Ohjelman rakenne: mistä moduuleista ja `.rs`-lähdekooditiedostoista projekti
  koostuu.
- Käytetyt kirjastot (crates) sekä lyhyt kuvaus siitä, miksi niitä tarvitaan.
- Miten samanaikaiset asiakkaat ja eri lähteistä tulevat samanaikaiset syötteet
  käsitellään palvelimessa ja asiakasohjelmassa.
- Tarkka **kuvaus projektissa käytetyistä protokollaviesteistä** (tärkeä).
- Käytetyt testauskäytännöt: millaisia yksikkötestejä, integraatiotestejä tai
  muita testejä on tehty.
- Kehitystyökalut ja työskentelytavat: ohjelmointiympäristön kuvaus, työnjako
  jos projekti tehtiin parityönä, sekä kuvaus käytetyistä tekoälytyökaluista ja
  siitä, miten niitä hyödynnettiin kehitystyön eri vaiheissa.
- Ohjeet toteutuksen suorittamiseen ja testaamiseen.
- Pohdinta projektin aikana opituista asioista ja lopputuloksen tunnetuista
  puutteista. Voit esimerkiksi kuvata, mitä tekisit toisin, jos aloittaisit
  uudelleen tai aikaa olisi ollut enemmän.
- Oma arviosi siitä, miten hyvin projekti onnistui täyttämään tällä sivulla
  kuvatut arviointiperusteet.

## Koodin selkeys ja rakenne (1)

- Koodi on jaettu loogisiin ja helposti ylläpidettäviin funktioihin.
- Koodi on jaettu loogisiin moduuleihin (sen sijaan, että kaikki olisi
  `main.rs`-tiedostossa).
- Koodin muotoilu sekä funktioiden ja tyyppien nimeämiskäytännöt ovat selkeitä
  ja johdonmukaisia. Rustissa on muotoilutyökalu, josta on tässä apua. Sen voi
  suorittaa esimerkiksi Cargon avulla komennolla `cargo fmt`.

## Toiminnallisuus (2)

- Asiakas- ja palvelintoteutuksissa ei ole merkittäviä virheitä, jotka estävät
  niiden normaalin käytön ja testaamisen.
- Palvelin toimii luotettavasti myös virhe- ja poikkeustilanteissa.
  Virheenkäsittely on asianmukaista eikä palvelimen suoritus keskeydy
  ajonaikaiseen paniikkiin.
- Palvelin käsittelee useita samanaikaisia asiakkaita riittävän nopeasti ilman
  liiallisia viiveitä.
- Toteutuksessa on yksikkö- ja integraatiotestejä, jotka kattavat erityisesti
  palvelimen keskeiset toiminnot.
- Tapahtumien jäljitys ja lokitus ovat riittäviä ja loogisesti jäsenneltyjä,
  esimerkiksi eri lokitustasoihin.

## Tiedonsiirto-ominaisuudet (1)

- TCP-yhteyksissä tuetaan TLS-suojausta. UDP:n kanssa ei tarvitse käyttää
  TLS:ää.
- Projektissa käytetään UDP-protokollaa jollakin mielekkäällä tavalla.
- Asiakas- ja palvelintoteutukset toimivat testatusti yhteen vähintään yhden
  muun, itsenäisesti kehitetyn saman aiheen toteutuksen kanssa.
  Yhteentoimivuuden ei tarvitse kattaa kaikkia ominaisuuksia, mutta sen tulee
  kattaa ainakin joitakin aiheen perustoimintoja (esimerkiksi valkotaulun
  perusobjektit tai autopelin sijaintipäivitykset).

Arviointi tehdään kokonaisuutena edellä kuvattujen osa-alueiden perusteella:
jos projekti jää joiltakin osin tavoitteista, puutteita voi korvata tekemällä
muita arvioinnin osia perusteellisemmin.
