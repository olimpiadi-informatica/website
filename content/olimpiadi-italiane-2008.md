+++
draft = false
title = "Olimpiadi italiane 2008"
date = 2008-04-07T05:58:31Z
updated = "2016-10-21T10:05:36"

[taxonomies]
gare = [ "nazionali",]
edizioni = [ "2008",]

[extra]
override_updated = "2016-10-21T10:05:36"
+++
## Regolamento

## INTERNATIONAL OLYMPIAD IN INFORMATICS- IOI 2008

<div style="text-align: center;">

Olimpiadi Italiane di Informatica

</div>

<div style="text-align: center;">

Regolamento gara nazionale 2008

</div>

<div style="text-align: center;">

**Pesaro/Fano, 4 aprile 2008**

</div>

<div style="text-align: center;">

</div>
Tipologia della gara

Obiettivo della gara è verificare le capacità dei partecipanti nel risolvere problemi mediante la scrittura di programmi secondo lo stile delle Olimpiadi Internazionali di Informatica (linguaggi, file input/output ecc.).

I problemi proposti, di difficoltà intermedia fra quella della selezione regionale e quella delle gare olimpiche internazionali, sono della stessa tipologia e sono formulati secondo quanto previsto dalle gare olimpiche.

Descrizione dei problemi

Ogni problema è caratterizzato dalle seguenti informazioni:

1. nome breve, che individua il problema;

2. coefficiente di difficoltà del problema (crescente con la difficoltà);

3. limiti di tempo massimo assegnato per l'esecuzione del programma (tale tempo è ampiamente maggiore di quello effettivamente necessario per la soluzione ottima);

4. descrizione del problema;

5. limiti e/o condizioni dei dati di ingresso;

6. esemplificazione attraverso un caso di prova;

7. eventuali note.

Soluzione dei problemi

I problemi devono essere risolti tramite programmi scritti in C/C++ o Pascal; le soluzioni devono funzionare correttamente con qualsiasi input che rispecchi i limiti assegnati nel testo.

I programmi devono obbligatoriamente leggere i dati in ingresso da un file di input di nome assegnato, e produrre i risultati su un file di output di nome assegnato; in particolare, questi file vanno aperti in C con le istruzioni

fr = fopen( "input.txt", "r" );

fw = fopen( "output.txt", "w" );

e in Pascal con le istruzioni

assign( fr, 'input.txt' ); reset( fr );

assign( fw, 'output.txt' ); rewrite( fw );

Il nome del file che contiene il programma deve essere esattamente il nome breve riportato nel testo del problema.

Il programma consegnatonon deve interagire in alcun modo con l'utente, né stampare dati non richiesti, anche se nello sviluppo del programma è ovviamente possibile utilizzare l'input/output da tastiera/video per eseguire testing e debugging.

I partecipanti possono scegliere quali problemi risolvere e in quale ordine.

Modalità di gara

Ciascun partecipante è identificato attraverso un documento di riconoscimento e gli viene assegnata una postazione di lavoro. I partecipanti hanno a disposizione 15 minuti per prendere visione dell'ambiente e per controllarne la corretta funzionalità.

All'inizio della gara saranno distribuiti i testi dei problemi stampati su carta. I programmi dovranno essere consegnati via rete mediante un sistema automatico (detto **sistema di sottoposizione**) che i partecipanti avranno sperimentato nel corso della prova del giorno precedente.

La durata della gara è di 5 ore. I partecipanti possono rivolgere alla commissione di sorveglianza solo domande di chiarimento scritte alle quali verrà data, sempre per iscritto, una delle seguenti risposte: SI, NO, RISPOSTA CONTENUTA NELLA DESCRIZIONE DEL PROBLEMA (esplicitamente o implicitamente), DOMANDA NON VALIDA, NO COMMENT (non si può rispondere).

Nessun partecipante può lasciare l’aula prima di un’ora e mezzo dall’inizio della prova. I testi dei problemi possono essere portati fuori dall’aula solo dopo il termine della prova.

Durante la gara è vietato portare con sé ed usare qualsiasi strumento di comunicazione interpersonale e di memorizzazione di informazioni, nonché consultare testi, manuali o appunti di qualsiasi genere, pena la squalifica.

Ambiente di programmazione

Ogni partecipante può scegliere fra le seguenti opzioni:

·Linux (distribuzione Ubuntu completa di gcc, g++, gdb, emacs, xemacs, vi, ddd) e FreePascal (fpc);

·Windows XP Professional, con gli ambienti di sviluppo della DevC++ e Dev-Pascal (FreePascal). Gli ambienti sono installati insieme al debugger grafico della Insight. Infine, è disponibile l'editor di testi NoteTab Light.

**I compilatori ufficiali della competizione sono il compilatore GNU per il C/C++ e il Free Pascal per il Pascal in ambiente Linux a 32 bit**. E’ anche possibile utilizzare l’ambiente Windows: in questo caso il sistema di sottoposizione segnalerà la presenza di eventuali non conformità con l’ambiente Linux e lo studente avrà la possibilità di apportare eventuali modifiche. Si sottolinea questa decisione poiché alcuni compilatori installati sotto Windows prevedono estensioni non-standard.

Si consiglia in ogni caso di:

1. utilizzare solo variabili intere a 32 bit (LongIntelong);

2.evitare l'uso di componenti non-standard, come lacrtdel Turbo Pascal o i file di intestazione, comeconio.hdel Turbo C;

3.compilare in ogni caso con il compilatore ufficiale prima della consegna mediante il sistema di sottoposizione.

Modalità di correzione e assegnazione dei punteggi

I partecipanti devono consegnare i sorgenti mediante il sistema di sottoposizione. Viene considerata valida l'ultima sottoposizione effettuata per ciascun problema. Il nome di ogni sorgente deve essere formato dal nome breve assegnato al problema, seguito dall'estensione relativa al linguaggio usato:.cper il C,.cppper il C++ e.pasper il Pascal.

Per la valutazione il programma sarà compilato ed eseguito su un insieme di casi di prova. Il punteggio finale consiste nella somma, pesata sul coefficiente di difficoltà del problema, dei punteggi ottenuti sui casi di prova. Lo stile di programmazione non ha alcun effetto sulla valutazione, così come il tempo necessario a risolvere ogni caso, purché rientri nei limiti previsti.

La classifica della gara sarà stilata in funzione del punteggio ottenuto dai singoli partecipanti. A parità di punteggio saranno favoriti i più giovani.

Premiazione e Probabili Olimpici 2008

In analogia a quanto accade nella gara internazionale, i primi classificati alle Olimpiadi Italiane di Informatica saranno premiati con:

MEDAGLIA D’ORO: dal primo al quinto classificato,

MEDAGLIA D’ARGENTO: dal sesto al quindicesimo classificato,

MEDAGLIA DI BRONZO: dal sedicesimo al trentacinquesimo classificato.

Il Comitato si riserva di apportare variazioni in caso di ex aequo.

I vincitori delle medaglie d’oro e d’argento saranno dichiarati Probabili Olimpici 2008 (PO2008) e saranno ammessi alle successive fasi di allenamento insieme a eventuali vincitori di medaglie di bronzo che, a insindacabile giudizio del Comitato, presentino particolari meriti.

IL COMITATO OLIMPICO

## Programma

ConfCommercio, Strada delle marche, 58 - Pesaro

ISIS "S. Marta", Strada delle marche, 1 - Pesaro

Teatro della Fortuna, Piazza XX settembre - Fano

## Atleti ammessi

**AMMESSI OLIMPIADI ITALIANE DI INFORMATICA 2008**

Come da regolamento sono ammessi alle Olimpiadi Italiane di Informatica:

1. i partecipanti agli stage di formazione della passata edizione che ancora frequentano la scuola ed hanno meno di 20 anni;
1. il primo classificato di ogni Regione con punteggio maggiore al valore medio della classifica nazionale (che è risultata di 11 punti);
1. i migliori della classifica nazionale, esclusi i primi per Regione, fino ad un massimo di 80 partecipanti complessivi.

|     |      |        |                       |               |                                   |                            |
| :-: | :--: | :----: | :-------------------: | :-----------: | :-------------------------------: | :------------------------: |
|     |
|  1  | EMI1 | PO2007 |      ELISABETTA       |   BERGAMINI   |            ITI GALILEI            |       MIRANDOLA (MO)       |
|  2  | VEN1 | PO2007 |        MATTEO         |   BOSCARIOL   |         ISS LUIGI EINAUDI         |     MONTEBELLUNA (TV)      |
|  3  | LOM1 | PO2007 |       EMANUELE        |    BRIVIO     |    ITI S. TEN. VASC. A. BADONI    |           LECCO            |
|  4  | LOM1 | PO2007 |        MASSIMO        |     CAIRO     |           L.S. MARCONI            |           MILANO           |
|  5  | LIG  | PO2007 |         PAOLO         |   COMASCHI    |        L.S. G. D. CASSINI         |           GENOVA           |
|  6  | EMI2 | PO2007 |        CLAUDIO        |   GUARISCO    |  ITI N.COPERNICO A. CARPEGGIANI   |          FERRARA           |
|  7  | EMI1 | PO2007 |         MARCO         |   LIPPARINI   |        ITI ETTORE MAJORANA        | SAN LAZZARO DI SAVENA (BO) |
|  8  | TOS  | PO2007 |       GIOVANNI        |  MASCELLANI   |           L.S. U. DINI            |            PISA            |
|  9  | PIE2 | PO2007 |        DAVIDE         |  PORTALUPPI   |           ITI G. FAUSER           |           NOVARA           |
| 10  | LOM2 | PO2007 |        RAMESH         |    RAJABY     |        ITI ETTORE MAJORANA        |        SERIATE (BG)        |
| 11  | SIC2 | PO2007 |       CALOGERO        |     RIZZO     |         FRANCESCO CRISPI          |        RIBERA (AG)         |
| 12  | VEN2 | PO2007 |        MANUELE        |   SABBADIN    |        ITI V. E. MARZOTTO         |       VALDAGNO (VI)        |
|     |
| 13  | ABR  |   23   |        LORENZO        |  DE LAURETIS  | ITI AMEDEO DI SAVOIA DUCA D'AOSTA |          L'AQUILA          |
| 14  | BAS  |   22   |        STEFANO        |   NICOLETTI   |          ITI PENTASUGLIA          |           MATERA           |
| 15  | CAL  |   17   |        CLAUDIO        |    FRATTO     |          ITI E. SCALFARO          |         CATANZARO          |
| 16  | CAM  |   24   |        GAETANO        |    PRISCO     |      L.S. DON CARLO LA MURA       |         ANGRI (SA)         |
| 17  | EMI  |   50   |       FRANCESCO       |     DONDI     |         L.S. ENRICO FERMI         |          BOLOGNA           |
| 18  | FRI  |   34   |        ANDREA         |    FOGARI     |              I.I.S.               |          GORIZIA           |
| 19  | LAZ  |   20   | VLADISLAV KRASSIMIROV |   VALTCHEV    |        L.S. INNOCENZO XII         |         ANZIO (RM)         |
| 20  | LIG  |   32   |        LORENZO        |   CENCESCHI   |      ITI GIOVANNI CAPELLINI       |         LA SPEZIA          |
| 21  | LOM  |   35   |       EMANUELE        |    BARBENO    |        ITI GALILEO GALILEI        |           CREMA            |
| 22  | MAR  |   30   |        ROBERTO        |   DOMENELLA   |           ITI E. MATTEI           |       RECANATI (MC)        |
| 23  | MOL  |   18   |       FEDERICO        |   CIMORELLI   |           ITI E. MATTEI           |          ISERNIA           |
| 24  | PIE  |   40   |        DAVIDE         |    BIANCO     |       ITI G. B. PININFARINA       |      MONCALIERI (TO)       |
| 25  | PUG  |   42   |      ALESSANDRO       |    GRASSI     |         L.S. L. DA VINCI          |        FASANO (BR)         |
| 26  | SAR  |   24   |       MAURIZIO        |    CARBONI    |            ITI OTHOCA             |          ORISTANO          |
| 27  | SIC  |   24   |        BENITO         |    GAMBINO    |     ITI VITTORIO EMANUELE III     |          PALERMO           |
| 28  | TOS  |   34   |        ANDREA         |   ESPOSITO    |             ITI FEDI              |          PISTOIA           |
| 29  | TRE  |   32   |        MATTEO         |    POLETTI    |      L.S. LEONARDO DA VINCI       |           TRENTO           |
| 30  | UMB  |   24   |       FRANCESCO       |    SBORGIA    |         L.S. R. DONATELLI         |           TERNI            |
| 31  | VEN  |   42   |         MIRKO         |   DA CORTE    |        ITI LUIGI NEGRELLI         |        FELTRE (BL)         |
|     |
| 32  | EMI2 |   42   |        MICHAEL        |    CAVINA     |         ITI NULLO BALDINI         |          RAVENNA           |
| 33  | VEN1 |   39   |       RICCARDO        |   MORANDIN    |          L.S. G. MARCONI          |      CONEGLIANO (TV)       |
| 34  | EMI1 |   38   |        MATTEO         |   PANCALDI    |        ITI F. ALBERGHETTI         |         IMOLA (BO)         |
| 35  | VEN2 |   38   |        ALBERTO        |     BEDIN     |         ITI G. CHILESOTTI         |        THIENE (VI)         |
| 36  | VEN2 |   38   |        DENNIS         |   OLIVETTI    |       ITI GUGLIELMO MARCONI       |           VERONA           |
| 37  | EMI2 |   36   |       GABRIELE        |   MONTUSCHI   |         ITI NULLO BALDINI         |          RAVENNA           |
| 38  | LOM1 |   35   |        ALBERTO        |   VETTOLANI   |    ITI S. TEN. VASC. A. BADONI    |           LECCO            |
| 39  | VEN2 |   35   |         MARCO         |    MACULAN    |         L.S. NICOLO' TRON         |         SCHIO (VI)         |
| 40  | FRI  |   34   |        PIETRO         |    CORONA     |            ITI KENNEDY            |         PORDENONE          |
| 41  | PIE2 |   34   |        DAVIDE         |  PORTALUPPI   |           ITI G. FAUSER           |           NOVARA           |
| 42  | VEN2 |   34   |       GIOVANNI        | DE FRANCESCHI |           L.S. GALILEI            |           VERONA           |
| 43  | PIE2 |   33   |      ALESSANDRO       |     DOVIS     |       L.S. GALILEO GALILEI        |        ALESSANDRIA         |
| 44  | LOM1 |   32   |         MARCO         |    CHIERA     |       ITC ALESSANDRO GREPPI       |  MONTICELLO BRIANZA (LC)   |
| 45  | VEN1 |   31   |         MARCO         |   FATTOREL    |           L.C. FLAMINIO           |    VITTORIO VENETO (TV)    |
| 46  | LOM1 |   30   |         LUCA          |    COLOMBO    |    ITI S. TEN. VASC. A. BADONI    |           LECCO            |
| 47  | LOM2 |   30   |        MATTIA         |    RIZZINI    |      ITI BENEDETTO CASTELLI       |          BRESCIA           |
| 48  | PIE1 |   30   |        ENRICO         |     PIOLA     |         ITC F. A. BONELLI         |           CUNEO            |
| 49  | VEN1 |   30   |       RICCARDO        |  FRANCESCHIN  |       L.S. GALILEO GALILEI        |         DOLO (VE)          |
| 50  | VEN1 |   30   |        ANDREA         |   VISENTIN    |          L.S. PRIMO LEVI          |     MONTEBELLUNA (TV)      |
| 51  | VEN2 |   29   |        ENRICO         |   DE GUIDI    |       ITI GUGLIELMO MARCONI       |           VERONA           |
| 52  | EMI2 |   28   |         MARCO         |   CASALBONI   |          ITI L. DA VINCI          |           RIMINI           |
| 53  | LOM1 |   28   |        ANDREA         |     ROSA'     |       ITI MAGISTRI CUMACINI       |            COMO            |
| 54  | VEN1 |   28   |       STANISLAV       |   SINIOUKOV   |          ITI E. BARSANTI          |  CASTELFRANCO VENETO (TV)  |
| 55  | FRI  |   27   |        MASSIMO        |     SECCI     |        L.S. M. GRIGOLETTI         |         PORDENONE          |
| 56  | LOM2 |   27   |        MICHAEL        |  FRANCHETTI   |        ITI ETTORE MAJORANA        |        SERIATE (BG)        |
| 57  | PIE1 |   27   |         MAURO         |     VIANO     |          ITI G.VALLAURI           |        FOSSANO (CN)        |
| 58  | LIG  |   26   |        ALESSIO        |    MERETA     |        L.S. G. D. CASSINI         |           GENOVA           |
| 59  | LOM2 |   26   |        NICOLA         |  BONIZZARDI   |      ITI BENEDETTO CASTELLI       |          BRESCIA           |
| 60  | PIE1 |   26   |        MATTIA         |   TRABUCCO    |          ITI G.VALLAURI           |        FOSSANO (CN)        |
| 61  | VEN1 |   26   |         JENNY         |    SPAGNOL    |         ISS LUIGI EINAUDI         |     MONTEBELLUNA (TV)      |
| 62  | VEN1 |   26   |        STEFANO        |   MONTAGNER   |        ITI CARLO  ZUCCANTE        |          VENEZIA           |
| 63  | VEN2 |   26   |      ALESSANDRO       |     MIMO      |               L.S.                |     CAMPOSAMPIERO (PD)     |
| 64  | FRI  |   25   |       FEDERICO        |    MUNINI     |         L.S. G. MARINELLI         |           UDINE            |
| 65  | LOM2 |   25   |        SIMONE         |    VOCELLA    |         ITI P. PALEOCAPA          |          BERGAMO           |
| 66  | MAR  |   25   |       FEDERICO        |    BAROCCI    |       ITI GUGLIELMO MARCONI       |         JESI (AN)          |
| 67  | VEN2 |   25   |      ALESSANDRO       |     BERTI     |        ITI M.O L. DAL CERO        |     SAN BONIFACIO (VR)     |
| 68  | EMI2 |   24   |         MARCO         |   DE CANAL    |      IIS MORCIANO DI ROMAGNA      |  MORCIANO DI ROMAGNA (RN)  |
| 69  | FRI  |   24   |        NICOLA         |     PAVAN     |        L.S. M. GRIGOLETTI         |         PORDENONE          |
| 70  | LIG  |   24   |        FILIPPO        | QUARIO RONDO  |      L.S. MARTIN LUTHER KING      |           GENOVA           |
| 71  | LOM2 |   24   |        NICOLA         |    ZAGHEN     |        IIS GRAZIO COSSALI         |       ORZINUOVI (BS)       |
| 72  | LOM2 |   24   |       MAURIZIO        |   ZUCCHELLI   |           L.S. LEONARDO           |          BRESCIA           |
| 73  | PUG1 |   24   |       VINCENZO        |    LUCENTE    |       ITI GUGLIELMO MARCONI       |            BARI            |
| 74  | PUG1 |   24   |       LEONARDO        |    PATIMO     |       ITI GALILEO FERRARIS        |       MOLFETTA (BA)        |
| 75  | SAR  |   24   |        DANIELE        |    SCHIRRU    |             ITI GIUA              |          CAGLIARI          |
| 76  | SIC2 |   24   |       RICCARDO        |    ANCONA     |          L.S. CANNIZZARO          |          PALERMO           |
| 77  | TOS  |   24   |        GIANNI         |   GENOVESI    |            ITI GALILEI            |          LIVORNO           |
| 78  | TOS  |   24   |       FRANCESCO       |    AZZURLI    |           L.S. F. REDI            |           AREZZO           |
| 79  | VEN2 |   24   |        MANUEL         |   CASTELLIN   |            ITI EUGANEO            |         ESTE (PD)          |
| 80  | TRE  |   23   |      MAXIMILIAN       |     ALBER     |          ITI MAX VALIER           |          BOLZANO           |

## Risultati

Le Olimpiadi Italiane 2008 si sono tenute nei giorni 3-5 aprile a Pesaro/Fano. Gli studenti vincitori delle medaglie d'oro e d'argento diventano Probabili Olimpici 2008 e risultano ammessi alla fase di allenamento.

Risultano inoltre ammessi alla formazione cinque vincitori di medaglie di bronzo scelti fra i più giovani.

I cinque studenti medagliati con l'oro alle ultime Olimpiadi Italiane di Informatica hanno vinto un premio della Banca d'Italia consistente in una borsa di studio da spendere per uno stage di due settimane che si terrà nel mese di settembre presso il laboratorio IBM di Hursley in Inghilterra.

| Medaglia |     |    Nome    |  Cognome   |            Istituto            |          Comune          | classe |
| :------: | :-: | :--------: | :--------: | :----------------------------: | :----------------------: | :----: |
|   ORO    |  1  |  MASSIMO   |   CAIRO    |          L.S. MARCONI          |          MILANO          |  III   |
|          |  2  |  GIOVANNI  | MASCELLANI |          L.S. U. DINI          |           PISA           |   V    |
|          |  3  |   RAMESH   |   RAJABY   |      ITI ETTORE MAJORANA       |       SERIATE (BG)       |   V    |
|          |  4  |   DENNIS   |  OLIVETTI  |     ITI GUGLIELMO MARCONI      |          VERONA          |   V    |
|          |  5  |   PAOLO    |  COMASCHI  |       L.S. G. D. CASSINI       |          GENOVA          |   IV   |
| ARGENTO  |  6  |  RICCARDO  |  MORANDIN  |        L.S. G. MARCONI         |     CONEGLIANO (TV)      |   IV   |
|          |  7  |  ALBERTO   |   BEDIN    |       ITI G. CHILESOTTI        |       THIENE (VI)        |   V    |
|          |  8  |  EMANUELE  |  BARBENO   |      ITI GALILEO GALILEI       |          CREMA           |   V    |
|          |  9  |   DAVIDE   | PORTALUPPI |         ITI G. FAUSER          |          NOVARA          |   V    |
|          | 10  |   MATTEO   | BOSCARIOL  |       ISS LUIGI EINAUDI        |    MONTEBELLUNA (TV)     |   V    |
|          | 11  |  DANIELE   |  SCHIRRU   |            ITI GIUA            |         CAGLIARI         |   IV   |
|          | 12  |   MAURO    |   VIANO    |         ITI G.VALLAURI         |       FOSSANO (CN)       |   V    |
|          | 13  |   BENITO   |  GAMBINO   |   ITI VITTORIO EMANUELE III    |         PALERMO          |   V    |
|          | 14  |  MAURIZIO  | ZUCCHELLI  |         L.S. LEONARDO          |         BRESCIA          |   IV   |
|          | 15  |  ALBERTO   | VETTOLANI  |  ITI S. TEN. VASC. A. BADONI   |          LECCO           |   V    |
|  BRONZO  | 16  |   ANDREA   |  VISENTIN  |        L.S. PRIMO LEVI         |    MONTEBELLUNA (TV)     |   V    |
|          | 17  | FRANCESCO  |   DONDI    |       L.S. ENRICO FERMI        |         BOLOGNA          |   IV   |
|          | 18  |   DAVIDE   |   BIANCO   |     ITI G. B. PININFARINA      |     MONCALIERI (TO)      |   V    |
|          | 19  |   SIMONE   |  VOCELLA   |        ITI P. PALEOCAPA        |         BERGAMO          |   V    |
|          | 20  |  CLAUDIO   |  GUARISCO  | ITI N.COPERNICO A. CARPEGGIANI |         FERRARA          |   V    |
|          | 21  |   MATTIA   |  TRABUCCO  |         ITI G.VALLAURI         |       FOSSANO (CN)       |   V    |
|          | 22  |  ROBERTO   | DOMENELLA  |         ITI E. MATTEI          |      RECANATI (MC)       |   IV   |
|          | 23  |  LORENZO   | CENCESCHI  |     ITI GIOVANNI CAPELLINI     |        LA SPEZIA         |   V    |
|          | 24  |   MARCO    |   CHIERA   |     ITC ALESSANDRO GREPPI      | MONTICELLO BRIANZA (LC)  |   IV   |
|          | 25  | MAXIMILIAN |   ALBER    |         ITI MAX VALIER         |         BOLZANO          |  III   |
|          | 26  |  FEDERICO  |   MUNINI   |       L.S. G. MARINELLI        |          UDINE           |   IV   |
|          | 27  |   MIRKO    |  DA CORTE  |       ITI LUIGI NEGRELLI       |       FELTRE (BL)        |   IV   |
|          | 28  |   MARCO    | CASALBONI  |        ITI L. DA VINCI         |          RIMINI          |   V    |
|          | 29  |  MAURIZIO  |  CARBONI   |           ITI OTHOCA           |         ORISTANO         |  III   |
|          | 30  |   NICOLA   |   ZAGHEN   |       IIS GRAZIO COSSALI       |      ORZINUOVI (BS)      |   V    |
|          | 31  |   ENRICO   |  DE GUIDI  |     ITI GUGLIELMO MARCONI      |          VERONA          |   V    |
|          | 32  | ALESSANDRO |   GRASSI   |        L.S. L. DA VINCI        |       FASANO (BR)        |   V    |
|          | 33  | STANISLAV  | SINIOUKOV  |        ITI E. BARSANTI         | CASTELFRANCO VENETO (TV) |   V    |
|          | 34  |  MICHAEL   |   CAVINA   |       ITI NULLO BALDINI        |         RAVENNA          |   IV   |
|          | 35  |   GIANNI   |  GENOVESI  |          ITI GALILEI           |         LIVORNO          |   IV   |

Prove assegnate

I testi assegnati alle Olimpiadi Italiane di Informatica ed. 2008

Esercizio 1: Troupe televisive (cnn)

Difficoltà D = 2 (tempo limite 2 sec).

Descrizione del problema

Mino ha deciso di intraprendere la carriera giornalistica iniziando il suo tirocinio negli Stati Uniti, presso la prestigiosa sede newyorkese della CNN. Il suo compito è quello di pianificare gli spostamenti giornalieri di due troupe televisive a Manhattan.

Com'è noto, le strade di Manhattan formano concettualmente una griglia di righe (_street_) e di colonne (_avenue_). La zona assegnata a Mino corrisponde a una griglia quadrata MxM, le cui righe e colonne sono entrambe numerate da 1 a M.

La CNN dispone di due troupe televisive, ciascuna dotata di una potente telecamera con zoom telescopico:

- la **troupe R** può muoversi soltanto lungo la prima colonna per riprendere ciò che succede nelle **righe** (street) della griglia;

- la **troupe C** può muoversi soltanto lungo la prima riga per riprendere ciò che avviene nelle **colonne** (avenue) della griglia.

Inizialmente, entrambe le troupe sono posizionate nell'incrocio che corrisponde alla prima riga e alla prima colonna.

Il **costo** di spostamente di una troupe, dalla posizione I alla posizione J, è misurato come il valore assoluto della differenza di posizione, ossia |I-J| (righe o colonne, a seconda della troupe). In questo modo, una troupe che non si sposta ha correttamente costo pari a zero.

Ogni mattina Mino riceve la lista degli N eventi che andranno ripresi nella giornata, nell'ordine temporale previsto (ossia il primo evento è il primo ad accadere e così via). Ciascun evento è identificato dalle sue coordinate r c a indicare che avverrà in corrispondenza dell'incrocio tra la riga (street) r e la colonna (avenue) c della griglia assegnata a Mino.

Tale evento potrà essere ripreso dalla troupe R, posizionandosi sulla riga r, oppure dalla troupe C, posizionandosi sulla colonna c. Per la troupe scelta da Mino per riprendere quell'evento, verrà pagato un costo pari al suo eventuale spostamento dalla posizione corrente alla posizione di ripresa (come definito sopra). Infatti, non è sempre necessario spostare una troupe per riprendere due eventi successivi.

Mino deve decidere quale delle due troupe va assegnata a ciascun evento da riprendere, in modo da minimizzare il costo totale, ovvero la somma dei costi di ognuna delle due troupe. Aiutate Mino a ottenere il costo totale minimo.

Dati di input

Il file input.txt è composto da N+1 righe, dove N è un intero positivo.

La prima riga contiene due interi positivi N e M che rappresentano rispettivamente il numero N di eventi da riprendere e il lato M della griglia (ossia ci sono M righe e M colonne).

Le successive N righe contengono gli eventi da riprendere nell'ordine temporale in cui si presenteranno. La k-esima di tali righe è composta da due interi r e c separati da uno spazio per indicare che il k-esimo evento (in ordine temporale) avverrà in corrispondenza dell'incrocio tra la riga (street) r e la colonna (avenue) c.

Dati di output

Il file output.txt è composto da N righe. La k-esima di tali righe contiene un solo carattere: la lettera R (maiuscola) oppure la lettera C (maiuscola) per indicare che il k-esimo evento (in ordine temporale) va ripreso, rispettivamente, con la troupe R oppure con la troupe C. La scelta operata in questo modo deve minimizzare il costo degli spostamenti delle due troupe per riprendere tutti gli eventi nel loro ordine dato.

Assunzioni

- 1 ≤ N, M ≤ 1000

- 1 ≤ r, c ≤ M

Esempi di input/output

|        File input.txt        | File output.txt |
| :--------------------------: | :-------------: |
| 3 5<br/>4 5 <br/>3 3<br/>2 2 |  R<br/>R<br/>C  |

|                       File input.txt                        |            File output.txt            |
| :---------------------------------------------------------: | :-----------------------------------: |
| 7 6<br/>4 2<br/>5 2<br/>6 2<br/>4 3<br/>4 4<br/>4 5<br/>4 6 | C<br/>C<br/>C<br/>R<br/>R<br/>R<br/>R |

### Nota/e

- Nel primo esempio sopra, il costo ottenuto da RRC è 5 (=3+1+1): in questo caso, tale costo è ottimo perché lo spostamento minimo per riprendere tutti gli eventi è proprio 5. Anche RRR è una risposta corretta in quanto il suo costo è 5. Bisogna specificarne una sola in output.
- Nel secondo esempio sopra, il costo ottenuto da CCCRRRR è ottimo perché è pari a 4 (=1+0+0+3+0+0+0). Notare che la sequenza CCCCCCC produce invece un costo non ottimo, in quanto esso è pari a 5 (=1+0+0+1+1+1+1).
- Se una troupe viene spostata su una riga (o colonna), vi rimane fino all'eventuale spostamento successivo, su indicazione di Mino.
- Un programma che restituisce sempre lo stesso valore, indipendentemente dai dati in input.txt, non totalizza alcun punteggio.

# Esercizio 2: Giornalismo d'inchiesta (triade)

## Difficoltà D = 2 (tempo limite 2 sec).

### Descrizione del problema

Durante la sua attività giornalistica, Mino incappa in uno scoop. Ha infatti scoperto l'esistenza di un'associazione che fu fondata e composta da due membri che si conoscevano direttamente. Tutti gli altri membri furono successivamente reclutati se conoscevano direttamente uno o due garanti facenti già parte dell'associazione.

Mino è riuscito a ricostruire la genesi dell'attuale associazione. Numerati i suoi membri da 1 a N, ha scritto sul suo taccuino una serie di M coppie di interi. In particolare, la coppia composta da due interi (I,J) indica che I ha garantito per J o viceversa. Infatti, Mino non è riuscito a stabilire chi dei due sia entrato per primo nell'associazione: sia (I,J) che (J,I) rappresentano la stessa coppia, anche perché l'ordine di numerazione dei membri non riflette necessariamente quello dell'iscrizione all'associazione (per esempio, non è detto che i membri numero 1 e 2 siano stati i fondatori o che il membro numero N sia l'ultimo arrivato).

Lo scoop di Mino consiste nell'aver trovato le prove di azioni criminali da parte di alcune triadi all'interno dell'associazione. Una triade è composta da tre membri I, J e K dell'associazione, tali che le coppie (I,J), (I,K) e (J,K) sono tutte presenti nel taccuino di Mino (è lecito supporre che i membri di una triade si siano aiutati reciprocamente, in un qualche ordine, per entrare nell'associazione).

Per stimare il numero di controlli che Mino deve effettuare, aiutatelo a contare efficientemente quante triadi in tutto contiene l'associazione, basandovi sulle coppie trascritte nel suo taccuino e sulla modalità di reclutamento di nuovi membri: per entrare nell'associazione occorre avere conoscenza diretta di uno o due garanti che ne siano già membri.

### Dati di input

Il file input.txt è composto da M+1 righe.

La prima riga contiene due interi positivi M e N separati da uno spazio: M rappresenta il numero di coppie contenute nel taccuino di Mino e N il numero di membri dell'associazione. Tali membri sono numerati da 1 a N.

Le successive M righe rappresentano le coppie nel taccuino: ciascuna riga è composta da due interi differenti I e J, separati da uno spazio, per indicare la coppia (I,J).

### Dati di output

Il file output.txt è composto da una sola riga contenente un intero non negativo che rappresenta il numero totale di triadi in seno all'associazione.

### Assunzioni

- 1 ≤ N ≤ 100000.
- N-1 ≤ M ≤ 2N-3.
- 1 ≤ I, J ≤ N.
- Non esistono due righe in input che rappresentano la stessa coppia. Notare che (I,J) = (J,I), per cui non è possibile trovarle entrambe in input.

### Esempi di input/output

|       File input.txt        | File output.txt |
| :-------------------------: | :-------------: |
| 3 4<br/>4 2<br/>1 3<br/>3 4 |        0        |

|                                                File input.txt                                                 | File output.txt |
| :-----------------------------------------------------------------------------------------------------------: | :-------------: |
| 13 8<br/>4 2<br/>8 3<br/>1 2<br/>8 5 <br/>6 8<br/>4 8<br/>7 2<br/>6 7<br/>2 8<br/>7 4<br/>8 1<br/>5 6<br/>3 2 |        5        |

### Nota/e

- L'ordine con cui si considerano i membri per definire una triade è irrilevante: per esempio, sia 2, 4, 7 che 4, 7, 2 definiscono la stessa triade e, pertanto, essa deve essere conteggiata una volta soltanto.
- Un programma che restituisce sempre lo stesso valore, indipendentemente dai dati in input.txt, non totalizza alcun punteggio.

# Esercizio 3: Parole saturnine (parole)

## Difficoltà D = 3 (tempo limite 1 sec).

### Descrizione del problema

Come ogni giornalista moderno che si rispetti, Mino deve imparare anche qualche lingua straniera. Mino, che è un tipo bizzarro, opta per il linguaggio saturnino! Su Saturno, ogni abitante ha un proprio vocabolario, che è formato dalle parole che non contengono una certa sequenza S di M caratteri consecutivi.

Per pura curiosità, Mino vuole contare quante parole sopravvivono in un tipico vocabolario saturnino. A tal fine, considera il caso più semplice delle parole su un alfabeto binario, aventi lunghezza prefissata N ≥ M, dove S è composta da M simboli binari.

Per esempio, se S = 01, allora le parole binarie di lunghezza N = 4 che non contengono S sono 0000, 1000, 1100, 1110, 1111. Se invece S = 11, allora esse sono 0000, 1000, 0100, 0010, 1010, 0001, 1001, 0101.

Mino vuole quindi contare le parole binarie di lunghezza N che non contengono S, solo che il loro numero può esplodere al crescere di N. Allora, indicato con K tale numero, Mino vuole calcolare il valore di K modulo 2011 (in effetti, gira voce che i saturnini sappiano solo contare da 0 fino a 2010 perché tale è il loro numero di dita). Aiutate Mino a calcolare tale valore.

### Dati di input

Il file input.txt è composto da due righe.

La prima riga contiene due interi positivi M e N separati da uno spazio: M rappresenta la lunghezza della sequenza binaria S da evitare e N la lunghezza delle parole binarie da contare.

La successiva riga contiene M caratteri binari '0' e '1' che rappresentano la sequenza binaria S da evitare.

### Dati di output

Il file output.txt è composto da una sola riga contenente un intero, compreso tra 0 e 2010, che rappresenta il valore K modulo 2011, dove K è il numero di parole binarie di lunghezza N che non contengono S.

### Assunzioni

- 1 ≤ N, M ≤ 1000.
- Per almeno metà dei casi di prova, su cui saranno valutati i programmi, vale 1 ≤ M ≤ 16.

### Esempi di input/output

| File input.txt | File output.txt |
| :------------: | :-------------: |
|   2 4<br/>01   |        5        |

| File input.txt | File output.txt |
| :------------: | :-------------: |
|   2 4<br/>11   |        8        |

| File input.txt | File output.txt |
| :------------: | :-------------: |
|  2 15<br/>11   |      1597       |

| File input.txt | File output.txt |
| :------------: | :-------------: |
|  2 16<br/>11   |       573       |

### Nota/e

- Ricordiamo che a modulo b = c se e solo se c è il resto della divisione intera tra a e b. L'operazione di modulo in linguaggio C si effettua con il simbolo di percentuale, in linguaggio Pascal con l'operatore mod.
- E' importante usare sempre il modulo dopo un'operazione aritmetica di conteggio perché i valori intermedi generati possono richiedere più di 32 bit. Poiché il risultato è K modulo 2011, suggeriamo di sfruttare il fatto che (A + B + C) modulo 2011 = ((A + B) modulo 2011) + C) modulo 2011. Stessa cosa per le altre operazioni aritmetiche. In questo modo, i risultati intermedi sono sempre modulo 2011 e possono essere mantenuti in una normale variabile intera a 32 bit.
- Se usate la piattaforma di sviluppo software basata sul compilatore Turbo Pascal e sul sistema operativo Windows, fare attenzione: i vostri programmi potrebbero essere valutati in una piattaforma diversa dalla vostra, e la garanzia di uniformità di comportamenti si ha soltanto se utilizzate sempre il tipo LongInt al posto del tipo Integer (quest'ultimo permette di rappresentare gli interi nell'intervallo [-32768...32767] mentre LongInt ne permette la rappresentazione in [-2147483648...2147483647]).
- Un programma che restituisce sempre lo stesso valore, indipendentemente dai dati in input.txt, non totalizza alcun punteggio.
