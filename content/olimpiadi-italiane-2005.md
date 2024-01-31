+++
draft = false
title = "Olimpiadi italiane 2005"
date = 2005-03-06T06:58:31Z
updated = "2016-10-21T11:07:27"

[taxonomies]
gare = [ "nazionali",]
edizioni = [ "2005",]

[extra]
override_updated = "2016-10-21T11:07:27"
+++

Le Olimpiadi Italiane di Informatica 2005 sono tenute nei giorni 3, 4 e 5 marzo presso l'ITCT "Salvatore Pugliatti" di Taormina.

<!-- more -->

## Regolamento

<div style="text-align: center;">

**INTERNATIONAL OLYMPIAD IN INFORMATICS – IOI 2005**

</div>

<div style="text-align: center;">

Regolamento

</div>

<div style="text-align: center;">

Olimpiadi Italiane di Informatica

</div>

# **Taormina, 4 marzo 2005**

<div style="text-align: center;">

</div>

**Tipologia della prova**

Obiettivo della prova è quello di verificare le capacità dei partecipanti di risolvere problemi tipici delle Olimpiadi di Informatica. I problemi proposti, di difficoltà intermedia fra quella della selezione regionale e quella delle prove olimpiche internazionali, sono della stessa tipologia e sono formulati secondo quanto previsto dalle prove olimpiche.

**Descrizione degli esercizi**

Ogni esercizio è caratterizzato dalle seguenti informazioni:

1. nome breve, che individua l'esercizio;

2. descrizione del problema;

3. esemplificazione attraverso un caso di prova;

4. limiti e/o condizioni dei dati di ingresso;

5. eventuali limiti di tempo massimo assegnato per l'esecuzione del programma (tale tempo è ampiamente maggiore di quello effettivamente necessario per la soluzione ottima);

6. coefficiente di difficoltà dell'esercizio (crescente con la difficoltà).

**Soluzione degli esercizi**

Gli esercizi devono essere risolti tramite programmi scritti in C/C++ o Pascal; le soluzioni devono funzionare correttamente con qualsiasi input che rispecchi i limiti assegnati nel testo.

I programmi devono obbligatoriamente leggere i dati in ingresso da un file di input di nome assegnato, e produrre i risultati su un file di output di nome assegnato. Il nome del file deve essere esattamente quello riportato nel tema, e cioè un file relativo alla directory corrente; letture da una directory prefissata porteranno alla squalifica del programma. I file vanno quindi aperti in C con le istruzioni

fr = fopen( "input.txt", "r" );

fw = fopen( "output.txt", "w" );

e in Pascal con le istruzioni

assign( fr, 'input.txt' ); reset( fr );

assign( fw, 'output.txt' ); rewrite( fw );

Il programma consegnato non deve interagire in alcun modo con l'utente, né stampare dati non richiesti, anche se nello sviluppo del programma è ovviamente possibile utilizzare l'input/output da tastiera/video per eseguire testing e debugging.

I partecipanti possono scegliere quali esercizi risolvere ed in che ordine.

**Ambiente di programmazione**

Ogni partecipante può scegliere fra le seguenti opzioni:

•Linux Fedora Core 2.0 completa di gcc, g++, gdb, emacs, xemacs, vi, ddd, ecc. e FreePascal (fpc);

•Windows XP Professional, con gli ambienti di sviluppo della DevC++ e Dev-Pascal (FreePascal). Gli ambienti sono installati insieme al debugger grafico della Insight. Infine, è disponibile l'editor di testi [NoteTab Light](http://www.notetab.com/).

Tutto il software finora descritto è scaricabile dal sito [http://www.olimpiadi-informatica.it](http://www.olimpiadi-informatica.it/)

**I compilatori ufficiali della competizione sono il compilatore GNU per il C/C++ e il Free Pascal per il Pascal**. Si sottolinea questa decisione poiché  alcuni  compilatori installati sotto Windows prevedono estensioni non-standard. I sorgenti sottoposti dai concorrenti verranno corretti utilizzando i compilatori prima specificati, in ambiente a 32 bit.

Si consiglia in ogni caso di:

1. Utilizzare solo variabili intere a 32 bit (LongInt e long);

2. Evitare l'uso di componenti non-standard, come la crt del Turbo Pascal o il file di intestazione conio.h del Turbo C;

3. Compilare in ogni caso prima della consegna con il compilatore ufficiale.

**Modalità di gara**

Ciascun partecipante è identificato attraverso un documento di riconoscimento e gli viene assegnata una postazione di lavoro. I partecipanti hanno a disposizione 30 minuti per prendere visione dell'ambiente e per controllare la corretta funzionalità dello stesso.

All'inizio della prova vengono consegnati loro i testi dei problemi stampati su carta; i programmi verranno sottoposti mediante un sistema di sottoposizione via rete che i partecipanti avranno già sperimentato nel corso della gara di prova del giorno precedente.

La durata della prova è di 5 ore; durante la prova, un partecipante può rivolgere alla commissione di sorveglianza domande di chiarimento scritte, alle quali potrà avere, sempre per iscritto, soltanto una delle seguenti risposte: SI, NO, NO COMMENT (non si può rispondere).

Durante la prova è vietato portare con sé telefoni cellulari, palmari e supporti elettronici, ottici e magnetici di memorizzazione nonché consultare testi, manuali o appunti di qualsiasi genere, pena esclusione dalla prova.

**Modalità di correzione della prova ed assegnazione punteggi**

I partecipanti devono consegnare i sorgenti sottoponendoli mediante il sistema automatico. Viene considerata valida l'ultima sottoposizione effettuata per ciascun problema. Il nome di ogni sorgente deve essere formato dal nome breve assegnato al problema, seguito dall'estensione propria del linguaggio: .c per il C, .cpp per il C++ e .pas per il Pascal.

Per la valutazione della prova il programma sarà compilato ed eseguito su un insieme di casi di prova. Il punteggio finale consiste nella somma dei punteggi ottenuti sui casi di prova. Il tempo necessario a risolvere ogni caso non è considerato ai fini della valutazione, purché rientri nei limiti previsti. Lo stile di programmazione non ha alcun effetto sulla valutazione.

Si fa presente che, pur potendo sviluppare indifferentemente i programmi sia in ambiente Windows che Linux, ai fini della valutazione le prove saranno ricompilate in ambiente Linux.

La classifica della prova sarà stilata in funzione del punteggio guadagnato dai singoli partecipanti. A parità di punteggio saranno favoriti i più giovani.

**Premiazione e Probabili Olimpici 2005**

In analogia a quanto si realizza per la gara internazionale, i primi 35 classificati alle Olimpiadi Italiane di Informatica saranno premiati con delle medaglie, e precisamente:

MEDAGLIA D’ORO: dal primo al quinto classificato,

MEDAGLIA D’ARGENTO: dal sesto al quindicesimo classificato,

MEDAGLIA DI BRONZO: dal sedicesimo al trentacinquesimo classificato.

I vincitori delle medaglie d’oro e d’argento saranno dichiarati Probabili Olimpici 2005 (PO2005) e saranno ammessi alla fase successiva.

IL COMITATO OLIMPICO

## Programma

<div style="text-align: center;">

{{ img(src='/images/uploads/programma2005.jpg', width='874', height='633') }}

</div>

## Atleti ammessi

Le Olimpiadi Italiane di Informatica 2005 si terranno nei giorni 3, 4 e 5 marzo presso l'ITCT "Salvatore Pugliatti" di Taormina

Risultano ammessi alle OII 2005:

- I partecipanti agli stage di formazione nel corso della passata edizione che ancora frequentano la scuola ed hanno meno di 20 anni. <br/>Per quest’anno detti studenti sono in numero di 5.
- Il primo classificato di ognuna delle 23 Sedi Regionali.
- I primi della classifica nazionale senza primi per sede che abbiano riportato un punteggio pari o superiore al 34.

|      |               |                  |                                              |                          |                 |           |
| :--: | :-----------: | :--------------: | :------------------------------------------: | :----------------------: | :-------------: | :-------: |
| LOM1 |   Barbieri    |       Luca       |          Liceo Scientifico Carducci          |          Milano          |   04/03/1987    |  PO 2004  |
| TRE  |   Guerrieri   |     Alessio      |     Liceo Scientifico Leonardo da Vinci      |          Trento          |   30/04/1986    |  PO 2004  |
| PIE1 |    Audrito    |     Giorgio      |          Liceo Scientifico M. Curie          |      Pinerolo (TO)       |   17/12/1986    |  PO 2004  |
| SIC1 |    Ingala     |    Salvatore     |       Istituto di Istruzione Superiore       |   Piazza Armerina (EN)   |   06/05/1987    |  PO 2004  |
| LIG  |   Canciani    |      Andrea      |       Liceo Scientifico G. D. Cassini        |          Genova          |   23/10/1986    |  PO 2004  |
|      |
| Sede |    Cognome    |       Nome       |                    Scuola                    |          Città           | Data di Nascita | Punteggio |
| ABR  |   Celestini   |    Valentina     |      ITI Amedeo di Savoia Duca d'Aosta       |         L'Aquila         |   04/03/1987    |    18     |
| CAL1 |    Mazzeo     |      Manuel      |       Istituto Scolastico Comprensivo        |  Oppido Mamertina (RC)   |   18/05/1988    |     2     |
| CAL2 |    Avolio     |    Alessandro    |         Istituto Tecnico Industriale         |       Rossano (CS)       |   20/07/1986    |    32     |
| CAM  | Scippacercola |      Fabio       |           Liceo Classico Garibaldi           |          Napoli          |   23/09/1988    |    58     |
| EMI  |   Catalano    |      Andrea      |             ITI Aldini Valeriani             |         Bologna          |   09/05/1986    |    52     |
| FRI  |   Contento    |     Lorenzo      |  Liceo Scientifico Michelangelo Buonarroti   |     Monfalcone (GO)      |   25/09/1988    |    48     |
| LAZ  |   Magaletti   |    Francesco     |          Liceo Scientifico Majorana          |          Latina          |   15/09/1986    |    80     |
| LIG  |    Presta     |    Alessandro    |       Liceo Scientifico G. D. Cassini        |          Genova          |   03/11/1987    |    49     |
| LOM1 |    Bonazzi    |    Francesco     |     Liceo Scientifico Leonardo da Vinci      |          Milano          |   07/03/1987    |    66     |
| LOM2 |   Pezzotti    |      Nicola      |            ITI Benedetto Castelli            |         Brescia          |   17/06/1986    |    48     |
| MAR  |    Spinaci    |      Marco       |        Liceo Scientifico Livio Cambi         | Falconara Marittima (AN) |   25/04/1986    |    58     |
| PIE1 |    Laviola    |    Francesco     |      Liceo Scientifico Galileo Ferraris      |          Torino          |   28/05/1987    |    56     |
| PIE2 |     Barba     |    Christian     |         Istituto Tecnico Industriale         |       Santhià (VC)       |   19/11/1988    |    47     |
| PUG1 |    Silvano    |     Giuseppe     |             ITI Sen. O. Iannuzzi             |       Andria (BA)        |   31/03/1987    |    48     |
| PUG2 |   Quartulli   |     Antonio      |                  ITI Giorgi                  |         Brindisi         |   03/10/1986    |    61     |
| SAR  |     Addis     |     Alberto      |          Liceo Scientifico G. Spano          |         Sassari          |   28/05/1987    |    28     |
| SIC1 |   Bonatesta   |    Francesco     | Liceo Scientifico Principe Umberto di Savoia |         Catania          |   05/04/1987    |    88     |
| SIC2 |    De Luca    |  Antonio Ercole  |          ITI Vittorio Emanuele III           |         Palermo          |   04/11/1987    |    26     |
| TOS  |     Dessì     |     Daniele      |                 ITI T. Buzzi                 |          Prato           |   29/01/1987    |    32     |
| TRE  |    Sartori    |     Antonio      |              Liceo Scientifico               |        Cles (TN)         |   21/12/1986    |    53     |
| UMB  |     Rossi     |     Alessio      |        Liceo Scientifico R. Donatelli        |          Terni           |   10/11/1986    |    32     |
| VEN1 |   Pierazzo    |      Nicola      |      Liceo Scientifico Galileo Galilei       |        Dolo (VE)         |   15/06/1987    |    76     |
| VEN2 |   Bonizzato   |      Marco       |      Liceo Scientifico Galileo Galilei       |          Verona          |   26/09/1986    |    66     |
|      |
| Sede |    Cognome    |       Nome       |                    Scuola                    |          Città           | Data di Nascita | Punteggio |
| VEN1 |   Ghirardo    |      Giulio      |                 IIS Flaminio                 |   Vittorio Veneto (TV)   |   15/03/1986    |    75     |
| VEN1 |  Bonaventura  |      Mattia      |              ITI Carlo Zuccante              |         Venezia          |   06/07/1986    |    61     |
| VEN2 |     Ferro     |     Daniele      |          ITI M. O. Luciano dal Cero          |    San Bonifacio (VR)    |   02/02/1986    |    59     |
| VEN1 |  Giacometti   |     Claudio      |         Istituto Tecnico Industriale         |          Padova          |   27/09/1986    |    58     |
| LOM1 |     Turri     |     Valerio      |              Liceo Scientifico               | San Donato Milanese (MI) |   01/09/1987    |    56     |
| VEN1 |    Zambon     |      Tobia       |              ITI Carlo Zuccante              |         Venezia          |   01/09/1986    |    56     |
| LAZ  |   Agostini    |    Alessandro    |      Liceo Scientifico Benedetto Croce       |           Roma           |   23/12/1986    |    52     |
| EMI  |  Cavicchioli  |     Roberto      |                 ITI F. Corni                 |          Modena          |   14/01/1986    |    52     |
| LAZ  | De Benedectis |      Flavio      |          Liceo Scientifico Enriques          |           Roma           |   17/12/1988    |    49     |
| PUG2 | Pietracaprina |    Francesca     |     Liceo Scientifico Leonardo da Vinci      |       Maglie (LE)        |   22/11/1987    |    48     |
| LAZ  |   Santaroni   |     Federico     |      Liceo Scientifico Francesco Severi      |        Frosinone         |   02/02/1986    |    48     |
| VEN1 |   Zuccollo    |      Andrea      |                ITI A. Meucci                 |     Cittadella (PD)      |   31/07/1987    |    47     |
| PUG2 |  De Matteis   |     Tiziano      |                ITI A. Meucci                 |      Casarano (LE)       |   16/01/1987    |    47     |
| LOM2 |    Bettani    |      Fabio       |      Liceo Scientifico Galileo Galilei       |     Caravaggio (BG)      |   08/11/1986    |    47     |
| VEN1 |  Calderolla   |     Giorgio      |              Liceo Scientifico               |     Conegliano (TV)      |   30/03/1986    |    47     |
| EMI  |   Cavallari   |     Tommaso      |      Liceo Scientifico Nicolò Copernico      |         Bologna          |   14/03/1988    |    46     |
| LOM1 |   Zaniboni    |     Stefano      |             ITI P. Hensemberger              |        Monza (MI)        |   17/08/1987    |    46     |
| VEN2 |   Banterle    |      Marco       |            ITI Guglielmo Marconi             |          Verona          |   24/06/1987    |    46     |
| FRI  |   Monteduro   |     Gabriele     |     Liceo Scientifico Guglielmo Oberdan      |         Trieste          |   20/05/1987    |    46     |
| TRE  |   Parmesan    |     Stefano      |            ITI Guglielmo Marconi             |      Rovereto (TN)       |   12/03/1987    |    46     |
| VEN1 |     Zago      |      Nicola      |              Liceo Scientifico               |    Montebelluna (TV)     |   06/12/1986    |    46     |
| LOM1 |    Aracne     |      Mattia      |              ITI C. Facchinetti              |     Castellanza (VA)     |   30/07/1987    |    44     |
| LOM1 |    Marini     |   Andrea Carlo   |     Liceo Scientifico Giuseppe Terragni      |   Olgiate Comasco (CO)   |   26/02/1987    |    44     |
| PUG1 |    Santoro    |     Giuseppe     |               ITC F. M. Genco                |      Altamura (BA)       |   24/03/1987    |    43     |
| SIC1 |    Epasto     |    Alessandro    |         Liceo Scientifico Archimede          |         Messina          |   02/11/1987    |    43     |
| VEN2 |   Meneguolo   |     Stefano      |      Liceo Scientifico Galileo Galilei       |         Belluno          |   10/05/1986    |    42     |
| VEN1 |    Calore     |      Giulio      |              ITI Carlo Zuccante              |         Venezia          |   29/07/1987    |    41     |
| VEN2 |   Dissegna    |     Stefano      |      Liceo Scientifico Jacopo da Ponte       | Bassano del Grappa (VI)  |   15/01/1987    |    41     |
| LAZ  |     Tomei     |      Paolo       |            ITC Leonardo da Vinci             |        Frosinone         |   29/12/1987    |    41     |
| VEN2 | De Franceschi |       Igor       |             ITI Silvio De Pretto             |        Schio (VI)        |   21/10/1987    |    40     |
| LOM2 |   Volonghi    |    Donatello     |      Liceo Scientifico Galileo Galilei       |     Caravaggio (BG)      |   31/10/1986    |    40     |
| EMI  |    Prandi     |      Dario       |       Liceo Scientifico Rinaldo Corso        |      Correggio (RE)      |   04/09/1986    |    40     |
| LOM2 |     Rota      |      Ivano       |            ITI Guglielmo Marconi             |       Dalmine (BG)       |   21/01/1986    |    40     |
| VEN1 |    Baccega    |      Andrea      |              ITI Vito Volterra               |  San Donà di Piave (VE)  |   16/09/1987    |    39     |
| LIG  |      Seu      |       Luca       |       Liceo Scientifico L. Lanfranconi       |          Genova          |   02/05/1986    |    39     |
| VEN2 |     Degan     |    Sebastiano    |            IIS Leonardo da Vinci             |        Cerea (VR)        |   13/10/1987    |    38     |
| VEN1 |    Masarin    |      Fabio       |              ITI Vito Volterra               |  San Donà di Piave (VE)  |   10/10/1987    |    38     |
| PIE2 |   Rigassio    |    Alessandro    |         Istituto Tecnico Industriale         |       Santhià (VC)       |   11/07/1986    |    38     |
| LAZ  |     Rudi      |    Alessandro    |          Liceo Scientifico Labriola          |           Roma           |   11/04/1986    |    38     |
| PUG1 |     Tria      | Fabrizio Antonio |             ITI Luigi dell'Erba              |  Castellana Grotte (BA)  |   28/11/1988    |    37     |
| VEN1 |   Virgulin    |      Marco       |       Liceo Scientifico Giordano Bruno       |       Mestre (VE)        |   13/07/1987    |    36     |
| VEN2 |    Grecchi    |     Filippo      |              ITI V. E. Marzotto              |      Valdagno (VI)       |   30/03/1987    |    36     |
| TRE  |  Pfeifhofer   |      Simon       |         Istituto Tecnico Industriale         |       Brunico (BZ)       |   22/07/1986    |    36     |
| CAM  |    Napoli     |      Davide      |               ITI B. Focaccia                |      Baronissi (SA)      |   20/02/1986    |    36     |
| VEN2 |    Ballis     |       Luca       |              ITI Luigi Negrelli              |       Feltre (BL)        |   23/03/1987    |    35     |
| EMI  |    Fiandro    |       Luca       |         Liceo Scientifico Aldo Moro          |      Reggio Emilia       |   12/06/1986    |    35     |
| PIE1 |   Graziano    |     Alessio      |          Liceo Scientifico G. Bruno          |          Torino          |   02/07/1987    |    34     |
| LOM1 |   Rainoldi    |      Fabio       |        Liceo Scientifico Arturo Tosi         |    Busto Arsizio (VA)    |   13/02/1987    |    34     |
| PUG1 |    Mattia     |       Elio       |                 ITI Panetti                  |           Bari           |   27/10/1986    |    34     |

## Risultati

Le Olimpiadi Italiane 2005 si sono tenute nei giorni 3, 4 e 5 marzo<br/>presso l'ITCeT Salvatore Pugliatti di Taormina (ME)<br/>

Gli studenti vincitori delle medaglie d'oro e d'argento diventano **Probabili Olimpionici 2005<br/>**e risultano ammessi alla fase di allenamento

|         |     |               |            |                                              |                          |
| :-----: | :-: | :-----------: | :--------: | :------------------------------------------: | :----------------------: |
|   ORO   |  1  |   Barbieri    |    Luca    |          Liceo Scientifico Carducci          |          Milano          |
|         |  2  |    Audrito    |  Giorgio   |          Liceo Scientifico M. Curie          |      Pinerolo (TO)       |
|         |  2  |   Canciani    |   Andrea   |       Liceo Scientifico G. D. Cassini        |          Genova          |
|         |  4  |    Bonazzi    | Francesco  |     Liceo Scientifico Leonardo da Vinci      |          Milano          |
|         |  5  |   Bonizzato   |   Marco    |      Liceo Scientifico Galileo Galilei       |          Verona          |
| ARGENTO |  6  |   Guerrieri   |  Alessio   |     Liceo Scientifico Leonardo da Vinci      |          Trento          |
|         |  7  |   Pierazzo    |   Nicola   |      Liceo Scientifico Galileo Galilei       |        Dolo (VE)         |
|         |  8  |    Spinaci    |   Marco    |        Liceo Scientifico Livio Cambi         | Falconara Marittima (AN) |
|         |  9  |     Barba     | Christian  |         Istituto Tecnico Industriale         |       Santhià (VC)       |
|         | 10  |    Ballis     |    Luca    |              ITI Luigi Negrelli              |       Feltre (BL)        |
|         | 11  |    Baccega    |   Andrea   |              ITI Vito Volterra               |  San Donà di Piave (VE)  |
|         | 12  |    Mattia     |    Elio    |                 ITI Panetti                  |           Bari           |
|         | 13  |    Ingala     | Salvatore  |       Istituto di Istruzione Superiore       |   Piazza Armerina (EN)   |
|         | 14  |   Quartulli   |  Antonio   |                  ITI Giorgi                  |         Brindisi         |
|         | 15  |   Pezzotti    |   Nicola   |            ITI Benedetto Castelli            |         Brescia          |
| BRONZO  | 16  |     Rudi      | Alessandro |          Liceo Scientifico Labriola          |           Roma           |
|         | 17  |   Magaletti   | Francesco  |          Liceo Scientifico Majorana          |          Latina          |
|         | 18  |   Bonatesta   | Francesco  | Liceo Scientifico Principe Umberto di Savoia |         Catania          |
|         | 19  |    Santoro    |  Giuseppe  |               ITC F. M. Genco                |      Altamura (BA)       |
|         | 20  |     Ferro     |  Daniele   |          ITI M. O. Luciano dal Cero          |    San Bonifacio (VR)    |
|         | 21  |  De Matteis   |  Tiziano   |                ITI A. Meucci                 |      Casarano (LE)       |
|         | 22  |     Degan     | Sebastiano |            IIS Leonardo da Vinci             |        Cerea (VR)        |
|         | 23  |    Epasto     | Alessandro |         Liceo Scientifico Archimede          |         Messina          |
|         | 24  |    Avolio     | Alessandro |         Istituto Tecnico Industriale         |       Rossano (CS)       |
|         | 25  |   Santaroni   |  Federico  |      Liceo Scientifico Francesco Severi      |        Frosinone         |
|         | 26  | Scippacercola |   Fabio    |           Liceo Classico Garibaldi           |          Napoli          |
|         | 27  |   Zuccollo    |   Andrea   |                ITI A. Meucci                 |     Cittadella (PD)      |
|         | 28  |    Fiandro    |    Luca    |         Liceo Scientifico Aldo Moro          |      Reggio Emilia       |
|         | 29  |   Contento    |  Lorenzo   |  Liceo Scientifico Michelangelo Buonarroti   |     Monfalcone (GO)      |
|         | 30  |   Rigassio    | Alessandro |         Istituto Tecnico Industriale         |       Santhià (VC)       |
|         | 31  |     Zago      |   Nicola   |              Liceo Scientifico               |    Montebelluna (TV)     |
|         | 32  |    Laviola    | Francesco  |      Liceo Scientifico Galileo Ferraris      |          Torino          |
|         | 33  |   Ghirardo    |   Giulio   |                 IIS Flaminio                 |   Vittorio Veneto (TV)   |
|         | 34  |  Calderolla   |  Giorgio   |              Liceo Scientifico               |     Conegliano (TV)      |
|         | 35  |      Seu      |    Luca    |       Liceo Scientifico L. Lanfranconi       |          Genova          |

## Prove assegnate

Prove assegnate
