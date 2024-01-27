+++
draft = false
title = "Olimpiadi italiane 2007"
date = 2007-03-09T06:58:31Z
updated = "2016-10-21T10:10:18"

[taxonomies]
gare = [ "nazionali",]
edition = [ "2007",]

[extra]
override_updated = "2016-10-21T10:10:18"
+++
## Regolamento

<div style="text-align: center;">

INTERNATIONAL OLYMPIAD IN INFORMATICS – IOI 2007

</div>

<div style="text-align: center;">

Regolamento

</div>

# Olimpiadi Italiane di Informatica

<div style="text-align: center;">

**Bari, 9 marzo 2007**

</div>
Tipologia della gara

Obiettivo della gara è quello di verificare le capacità dei partecipanti di risolvere problemi tipici delle Olimpiadi di Informatica. I problemi proposti, di difficoltà intermedia fra quella della selezione regionale e quella delle gare olimpiche internazionali, sono della stessa tipologia e sono formulati secondo quanto previsto dalle gare olimpiche.

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

Il nome del file che contiene il programma deve essere esattamente il nome breve riportato nel testo del problema perchè in sede di correzione verrà considerato che esso sia contenuto nella directory corrente; letture da una directory prefissata porteranno all’annullamento del problema.

Il programma consegnato non deve interagire in alcun modo con l'utente, né stampare dati non richiesti, anche se nello sviluppo del programma è ovviamente possibile utilizzare l'input/output da tastiera/video per eseguire testing e debugging.

I partecipanti possono scegliere quali problemi risolvere e in quale ordine.

Modalità di gara

Ciascun partecipante è identificato attraverso un documento di riconoscimento e gli viene assegnata una postazione di lavoro. I partecipanti hanno a disposizione 30 minuti per prendere visione dell'ambiente e per controllarne la corretta funzionalità.

All'inizio della gara vengono consegnati i testi dei problemi stampati su carta; i programmi verranno consegnati via rete mediante un sistema automatico (detto sistema di sottoposizione) che i partecipanti avranno sperimentato nel corso della prova del giorno precedente.

La durata della gara è di 5 ore; durante la gara, un partecipante può rivolgere alla commissione di sorveglianza solo domande di chiarimento scritte, alle quali riceverà, sempre per iscritto, una delle seguenti risposte: SI, NO, NO COMMENT (non si può rispondere).

Durante la gara è vietato portare con sé ed usare qualsiasi strumento di comunicazione interpersonale e di memorizzazione di informazioni, nonché consultare testi, manuali o appunti di qualsiasi genere, pena la squalifica.

Ambiente di programmazione

Ogni partecipante può scegliere fra le seguenti opzioni:

● Linux (Ubuntu 6.06 LTS Dapper Drake completa di gcc, g++, gdb, emacs, xemacs, vi, ddd) e FreePascal (fpc);

● Windows XP Professional, con gli ambienti di sviluppo della DevC++ e Dev-Pascal (FreePascal). Gli ambienti sono installati insieme al debugger grafico della Insight. Infine, è disponibile l'editor di testi NoteTab Light.

I compilatori ufficiali della competizione sono il compilatore GNU per il C/C++ e il Free Pascal per il Pascal in ambiente Linux a 32 bit. E’ anche possibile utilizzare l’ambiente Windows: in questo caso il sistema di sottoposizione segnalerà la presenza di eventuali non conformità con l’ambiente Linux e lo studente avrà la possibilità di apportare eventuali modifiche. Si sottolinea questa decisione poiché alcuni compilatori installati sotto Windows prevedono estensioni non-standard.

Si consiglia in ogni caso di:

1. utilizzare solo variabili intere a 32 bit (LongInt e long);

2. evitare l'uso di componenti non-standard, come la crt del Turbo Pascal o il file di intestazione conio.h del Turbo C;

3. compilare in ogni caso con il compilatore ufficiale prima della consegna mediante il sistema di sottoposizione.

Modalità di correzione e assegnazione dei punteggi

I partecipanti devono consegnare i sorgenti mediante il sistema di sottoposizione. Viene considerata valida l'ultima sottoposizione effettuata per ciascun problema. Il nome di ogni sorgente deve essere formato dal nome breve assegnato al problema, seguito dall'estensione relativa al linguaggio usato: .c per il C, .cpp per il C++ e .pas per il Pascal.

Per la valutazione il programma sarà compilato ed eseguito su un insieme di casi di prova. Il punteggio finale consiste nella somma, pesata sul coefficiente di difficoltà del problema, dei punteggi ottenuti sui casi di prova. Lo stile di programmazione non ha alcun effetto sulla valutazione, così come il tempo necessario a risolvere ogni caso, purché rientri nei limiti previsti.

La classifica della gara sarà stilata in funzione del punteggio ottenuto dai singoli partecipanti. A parità di punteggio saranno favoriti i più giovani.

Premiazione e Probabili Olimpici 2007

In analogia a quanto accade nella gara internazionale, i primi 35 classificati alle Olimpiadi Italiane di Informatica saranno premiati con delle medaglie, e precisamente:

MEDAGLIA D’ORO: dal primo al quinto classificato,

MEDAGLIA D’ARGENTO: dal sesto al quindicesimo classificato,

MEDAGLIA DI BRONZO: dal sedicesimo al trentacinquesimo classificato.

Il Comitato si riserva di apportare variazioni in caso di ex aequo.

I vincitori delle medaglie d’oro e d’argento saranno dichiarati Probabili Olimpici 2007 (PO2007) e saranno ammessi alla fase successiva.

IL COMITATO OLIMPICO

## Programma

## Il Programma della manifestazione

<div style="text-align: center;">

{{ img(src='/images/uploads/programmaOII2007.png', width='800', height='566') }}

</div>

## Atleti ammessi

## AMMESSI OLIMPIADI ITALIANE DI INFORMATICA 2007

Visti i risultati della selezione regionale e indipendentemente dall'errore nell'esempio dell'esercizio Torero Escamillo considerando il quale la graduatoria non subisce variazioni, il Comitato Olimpico ha deciso di ammettere alle Olimpiadi Italiane di Informatica che si terranno a Bari nei giorni 8-9-10 marzo 2007 i seguenti studenti:

|     |                       |        |                  |               |                                              |                              |
| :-: | :-------------------: | :----: | :--------------: | :-----------: | :------------------------------------------: | :--------------------------: |
|     |                       |
|  1  |          CAM          | PO2006 |      Fabio       | Scippacercola |           Liceo Classico Garibaldi           |            Napoli            |
|  2  |          EMI          | PO2006 |     Tommaso      |   Cavallari   |      Liceo Scientifico Nicolò Copernico      |           Bologna            |
|  3  |         PUG2          | PO2006 |      Matteo      |  Campanelli   |        Liceo Scientifico Enrico Fermi        |           Brindisi           |
|  4  |          TRE          | PO2006 |     Giovanni     |   Angelini    |      Liceo Scientifico Galileo Galilei       |            Trento            |
|  5  |         VEN1          | PO2006 |      Denis       |    Nardin     |      Liceo Scientifico Ettore Maiorana       |         Mirano (VE)          |
|     |                       |
|  6  |        Abruzzo        |   18   |      Boris       |     Mazza     |                ITIS A. Volta                 |           Pescara            |
|  7  |      Basilicata       |   24   |     Michele      |    Cotrufo    |      Liceo Scientifico Dante Alighieri       |            Matera            |
|  8  |    Calabria (CAL1)    |   20   |     Raffaele     |    Faragò     |               ITIS E. Scalfaro               |          Catanzaro           |
|  9  |       Campania        |   28   |     Pasquale     |   Radunanza   |               ITI E. Majorana                |     Somma Vesuviana (NA)     |
| 10  |    Emilia Romagna     |   34   |      Marco       |   Lipparini   |        Liceo Scientifico Enrico Fermi        |           Bologna            |
| 11  | Friuli Venezia Giulia |   26   |     Lorenzo      |   Contento    |  Liceo Scientifico Michelangelo Buonarroti   |       Monfalcone (GO)        |
| 12  |         Lazio         |   46   |      Flavio      | De Benedictis |          Liceo Scientifico Enriques          |             Roma             |
| 13  |        Liguria        |   18   |      Paolo       |   Comaschi    |       Liceo Scientifico G. D. Cassini        |            Genova            |
| 14  |   Lombardia (LOM1)    |   48   |     Massimo      |     Cairo     |          Liceo Scientifico Marconi           |            Milano            |
| 15  |        Marche         |   32   |     Federico     |    Barocci    |            ITI Guglielmo Marconi             |          Jesi (AN)           |
| 16  |    Piemonte (PIE2)    |   28   |    Christian     |     Barba     |         Istituto Tecnico Industriale         |         Santhià (VC)         |
| 17  |     Puglia (PUG2)     |   46   |     Vincenzo     |   Maffione    |              ITC Blaise Pascal               |            Foggia            |
| 18  |       Sardegna        |   18   |       Luca       |    Maddau     |               ITI G. M. Angioy               |           Sassari            |
| 19  |    Sicilia (SIC1)     |   32   |    Salvatore     |     Aiola     | Liceo Scientifico Principe Umberto di Savoia |           Catania            |
| 20  |        Toscana        |   22   |     Giovanni     |  Mascellani   |          Liceo Scientifico U. Dini           |             Pisa             |
| 21  |  Trentino Alto Adige  |   36   |      Julian      |    Mautner    |      Liceo Scientifico Albert Einstein       |         Merano (BZ)          |
| 22  |        Umbria         |   20   |     Lorenzo      |  Verstraeten  |        Liceo Scientifico R. Donatelli        |            Terni             |
| 23  |     Veneto (VEN1)     |   48   |     Roberto      |  Compostella  |              ITCG Luigi Einaudi              |      Montebelluna (TV)       |
|     |                       |
| 24  |         VEN2          |   44   |    Francesco     |    Piccoli    |          Liceo Scientifico Galilei           |            Verona            |
| 25  |         LOM2          |   42   |      Davide      |   Galliani    |      Liceo Scientifico Galileo Galilei       |       Caravaggio (BG)        |
| 26  |         VEN1          |   38   |     Riccardo     |   Morandin    |                Liceo Statale                 |       Conegliano (TV)        |
| 27  |          EMI          |   34   |      Marco       |    Poletti    |             ITIS Galileo Galilei             |  San Secondo Parmense (PR)   |
| 28  |         VEN2          |   34   |     Riccardo     |     Rigon     |            ITI Guglielmo Marconi             |            Verona            |
| 29  |          EMI          |   32   |     Claudio      |   Guarisco    |        ITI N.Copernico A. Carpeggiani        |           Ferrara            |
| 30  |         LOM2          |   32   |      Ramesh      |    Rajaby     |         Istituto Tecnico Industriale         |         Seriate (BG)         |
| 31  |         PUG2          |   32   |     Tommaso      |     Adamo     |                ITI A. Meucci                 |        Casarano (LE)         |
| 32  |         PUG2          |   32   |     Stefano      |    Leucci     |            ITC A. Cezzi de Castro            |         Maglie (LE)          |
| 33  |          TRE          |   32   |      Markus      |     Amhof     |         Istituto Tecnico Industriale         |         Brunico (BZ)         |
| 34  |         VEN1          |   32   |     Daniele      |  Brazzolotto  |                ITI Primo Levi                |         Mirano (VE)          |
| 35  |          EMI          |   30   |      Andrea      |  Signifredi   |             ITIS Galileo Galilei             |  San Secondo Parmense (PR)   |
| 36  |         LOM2          |   30   |     Roberto      |    Codazzi    |            ITI Benedetto Castelli            |           Brescia            |
| 37  |         VEN2          |   30   |     Manuele      |   Sabbadin    |              ITI V. E. Marzotto              |        Valdagno (VI)         |
| 38  |          EMI          |   28   |      Andrea      |     Asta      |             ITI Aldini Valeriani             |           Bologna            |
| 39  |         PUG1          |   28   | Fabrizio Antonio |     Tria      |             ITI Luigi Dell'Erba              |    Castellana Grotte (BA)    |
| 40  |         VEN2          |   28   |      Andrea      |   Pegoraro    |        Liceo Scientifico Nicolò Tron         |          Schio (VI)          |
| 41  |         LOM2          |   26   |     Stefano      |     Riva      |         ITI S. Ten. Vasc. A. Badoni          |            Lecco             |
| 42  |         PIE2          |   26   |      Paolo       |    Pasini     |             ITI Ascanio Sobrero              |    Casale Monferrato (AL)    |
| 43  |         VEN2          |   26   |      Loris       |    Securo     |                  ITI Meucci                  |       Cittadella (PD)        |
| 44  |         VEN2          |   26   |    Alessandro    |   Ambrosano   |            ITI Guglielmo Marconi             |            Verona            |
| 45  |         VEN2          |   26   |     Michele      |   Zerbetto    |            ITI Guglielmo Marconi             |            Verona            |
| 46  |          EMI          |   24   |     Riccardo     |    Monica     |          Liceo Scientifico Marconi           |            Parma             |
| 47  |         LOM1          |   24   | Claudio Alberto  |   Andreoni    |      Liceo Scientifico Maria Immacolata      |            Milano            |
| 48  |         SIC2          |   24   |     Calogero     |     Rizzo     |     Istituto Magistrale Francesco Crispi     |         Ribera (AG)          |
| 49  |          TRE          |   24   |     Siegmar      |     Alber     |         Istituto Tecnico Industriale         |        Silandro (BZ)         |
| 50  |         VEN2          |   24   |      Andrea      |   Bassetto    |              ITI G. Chilesotti               |         Thiene (VI)          |
| 51  |         VEN2          |   24   |      Marco       |   Tamassia    |            ITI Guglielmo Marconi             |            Verona            |
| 52  |         VEN1          |   22   |     Emanuele     | Franceschini  |              ITI Carlo Zuccante              |           Venezia            |
| 53  |          CAM          |   20   |     Daniele      |   Lombardi    |                 ITI R. Elia                  | Castellammare di Stabia (NA) |
| 54  |          EMI          |   20   |     Gianluca     |     Dotti     |                 ITI Galilei                  |        Mirandola (MO)        |
| 55  |          EMI          |   20   |    Elisabetta    |   Bergamini   |                 ITI Galilei                  |        Mirandola (MO)        |
| 56  |         LOM1          |   20   |     Daniele      |   Sempione    |                ITI G. Marconi                |       Gorgonzola (MI)        |
| 57  |         LOM1          |   20   |     Roberto      |     Caron     |               ITI Marie Curie                |            Milano            |
| 58  |         LOM1          |   20   |     Stefano      |   Schiavoni   |          Liceo Scientifico da Vinci          |            Milano            |
| 59  |         LOM1          |   20   |     Daniele      |     Sala      |             ITI P. Hensemberger              |          Monza (MI)          |
| 60  |         LOM1          |   20   |      Davide      |   Locatelli   |             ITI P. Hensemberger              |          Monza (MI)          |
| 61  |         LOM2          |   20   |      Enrico      |    Gaspani    |             ITI Gugliemo Marconi             |         Dalmine (BG)         |
| 62  |         LOM2          |   20   |     Alberto      |   Vettolani   |         ITI S. Ten. Vasc. A. Badoni          |            Lecco             |
| 63  |         LOM2          |   20   |     Emanuele     |    Brivio     |         ITI S. Ten. Vasc. A. Badoni          |            Lecco             |
| 64  |         LOM2          |   20   |     Riccardo     |    Grandi     |                ITIS E. Fermi                 |           Mantova            |
| 65  |         PIE2          |   20   |      Davide      |  Portaluppi   |                ITI G. Fauser                 |            Novara            |
| 66  |         PUG1          |   20   |      Nunzio      |  Gianfelice   |                 ITC Vivante                  |             Bari             |
| 67  |         PUG1          |   20   |     Gianluca     |   Novembre    |             ITI Luigi Dell'Erba              |    Castellana Grotte (BA)    |
| 68  |          TOS          |   20   |  Luigi Federico  |     Rossi     |          Liceo Scientifico F. Redi           |            Arezzo            |
| 69  |          TOS          |   20   |      Matteo      |  Razzanelli   |          ITI Ferraris Brunelleschi           |         Empoli (FI)          |
| 70  |          TRE          |   20   |      Stefan      |    Theiner    |                ITI Max Valier                |           Bolzano            |
| 71  |         VEN1          |   20   |    Francesco     |      Lin      |         Liceo Scientifico G. Galilei         |           Belluno            |
| 72  |         VEN1          |   20   |      Matteo      |   Boscariol   |              ITCG Luigi Einaudi              |      Montebelluna (TV)       |
| 73  |         VEN2          |   20   |    Francesco     |     Rossi     |            ITIS Francesco Severi             |            Padova            |
| 74  |         VEN2          |   20   |      Denis       |   Tramonte    |                 ITI C. Anti                  |  Villafranca di Verona (VR)  |

## Risultati

Le Olimpiadi Italiane 2007 si sono tenute nei giorni 8-10 marzo a Bari. Gli studenti vincitori delle medaglie d'oro e d'argento diventano Probabili Olimpionici 2007 e risultano ammessi alla fase di allenamento.<br/>Risultano inoltre ammessi alla formazione anche i vincitori delle medaglie di bronzo che frequentano le classi inferiori alla quinta superiore.

|         |     |            |              |                                           |                   |
| :-----: | :-: | :--------: | :----------: | :---------------------------------------: | :---------------: |
|   Oro   |  1  |  Massimo   |    Cairo     |         Liceo Scientifico Marconi         |      Milano       |
|         |  2  |  Claudio   |   Guarisco   |      ITI N.Copernico A. Carpeggiani       |      Ferrara      |
|         |  3  |   Julian   |   Mautner    |     Liceo Scientifico Albert Einstein     |    Merano (BZ)    |
|         |  4  |  Giovanni  |  Mascellani  |         Liceo Scientifico U. Dini         |       Pisa        |
|         |  5  |  Manuele   |   Sabbadin   |            ITI V. E. Marzotto             |   Valdagno (VI)   |
| Argento |  6  |   Matteo   |  Boscariol   |            ITCG Luigi Einaudi             | Montebelluna (TV) |
|         |  7  |  Roberto   |   Codazzi    |          ITI Benedetto Castelli           |      Brescia      |
|         |  8  |   Davide   |   Galliani   |     Liceo Scientifico Galileo Galilei     |  Caravaggio (BG)  |
|         |  9  |   Paolo    |   Comaschi   |      Liceo Scientifico G. D. Cassini      |      Genova       |
|         | 10  |  Emanuele  | Franceschini |            ITI Carlo Zuccante             |      Venezia      |
|         | 11  |  Giovanni  |   Angelini   |     Liceo Scientifico Galileo Galilei     |      Trento       |
|         | 11  |  Stefano   |    Leucci    |          ITC A. Cezzi de Castro           |    Maglie (LE)    |
|         | 13  |  Tommaso   |  Cavallari   |    Liceo Scientifico Nicolò Copernico     |      Bologna      |
|         | 13  |   Ramesh   |    Rajaby    |       Istituto Tecnico Industriale        |   Seriate (BG)    |
|         | 15  |  Roberto   | Compostella  |            ITCG Luigi Einaudi             | Montebelluna (TV) |
| Bronzo  | 16  |  Tommaso   |    Adamo     |               ITI A. Meucci               |   Casarano (LE)   |
|         | 16  |  Gianluca  |    Dotti     |                ITI Galilei                |  Mirandola (MO)   |
|         | 16  |  Calogero  |    Rizzo     |   Istituto Magistrale Francesco Crispi    |    Ribera (AG)    |
|         | 16  |  Michele   |   Zerbetto   |           ITI Guglielmo Marconi           |      Verona       |
|         | 20  |  Siegmar   |    Alber     |       Istituto Tecnico Industriale        |   Silandro (BZ)   |
|         | 20  | Elisabetta |  Bergamini   |                ITI Galilei                |  Mirandola (MO)   |
|         | 20  |  Daniele   | Brazzolotto  |              ITI Primo Levi               |    Mirano (VE)    |
|         | 20  |  Riccardo  |    Grandi    |               ITIS E. Fermi               |      Mantova      |
|         | 24  | Francesco  |     Lin      |       Liceo Scientifico G. Galilei        |      Belluno      |
|         | 24  |  Vincenzo  |   Maffione   |             ITC Blaise Pascal             |      Foggia       |
|         | 24  |  Stefano   |     Riva     |        ITI S. Ten. Vasc. A. Badoni        |       Lecco       |
|         | 27  | Christian  |    Barba     |       Istituto Tecnico Industriale        |   Santhià (VC)    |
|         | 27  |   Davide   |  Portaluppi  |               ITI G. Fauser               |      Novara       |
|         | 29  |   Loris    |    Securo    |                ITI Meucci                 |  Cittadella (PD)  |
|         | 30  |  Emanuele  |    Brivio    |        ITI S. Ten. Vasc. A. Badoni        |       Lecco       |
|         | 30  |  Lorenzo   |   Contento   | Liceo Scientifico Michelangelo Buonarroti |  Monfalcone (GO)  |
|         | 32  |   Enrico   |   Gaspani    |           ITI Gugliemo Marconi            |   Dalmine (BG)    |
|         | 33  |   Andrea   |   Bassetto   |             ITI G. Chilesotti             |    Thiene (VI)    |
|         | 33  |  Riccardo  |    Monica    |         Liceo Scientifico Marconi         |       Parma       |
|         | 35  |   Marco    |  Lipparini   |      Liceo Scientifico Enrico Fermi       |      Bologna      |

## Prove assegnate

Queste le prove assegnate alle Olimpiadi Italiane di Informatica ed. 2007.

[Domina](/oldsite/81/domina.pdf)

[Petali](/oldsite/81/petali.pdf)

[Somme](/oldsite/81/somme.pdf)
