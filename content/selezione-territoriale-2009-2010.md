+++
draft = false
title = "Selezione territoriale 2009-2010"
date = 2010-04-20T11:50:29.000Z
updated = "2016-10-20T13:35:04"
description = "selezione territoriale regionale 2009-2010"

[taxonomies]
edition = ["2009-2010"]
gare = ["territoriali"]

[extra]
override_updated = "2016-10-20T13:35:04"
+++
La selezione territoriale delle Olimpiadi di Informatica si è svolta il 14 aprile 2010. Sono ammessi alla fase successiva 79 studenti provenienti da tutta Italia.

<!-- more -->

## Regolamento

<div style="text-align: center;">

**INTERNATIONAL OLYMPIAD IN INFORMATICS**

**Olimpiadi Italiane di Informatica**

**Regolamento della gara di selezione territoriale del 14 aprile 2010**

**(IOI 2011)**

</div>

***1. Ammissione alla gara di selezione***

Per partecipare lo studente deve:

• avere superato la selezione scolastica,

• essere disponibile, qualora superi la selezione territoriale, a partecipare alla gara nazionale ed a frequentare i corsi di formazione che si terranno prima della prova internazionale,

• essere disponibile, qualora superi l’ultima selezione, a recarsi all’estero nel periodo della gara internazionale con gli accompagnatori designati.

***2. Caratteristiche della gara***

Obiettivo della gara è verificare le capacità dei partecipanti nel risolvere problemi mediante la scrittura di programmi secondo lo stile delle Olimpiadi Internazionali di Informatica. I problemi proposti sono simili a quelli proposti in sede internazionale, anche se mediamente più semplici.

***3. Descrizione dei problemi***

La gara consiste nella soluzione di tre problemi, per ognuno dei quali sono fornite le seguenti informazioni:

• nome breve, che individua il problema;

• coefficiente di difficoltà D (crescente con la difficoltà)

• descrizione del problema

• limiti e/o condizioni dei dati di ingresso

• esemplificazione per mezzo di uno o più casi di prova

• eventuali note.

***4. Soluzione dei problemi***

I problemi devono essere risolti tramite programmi scritti in linguaggio C/C++ o in linguaggio Pascal; tali programmi devono funzionare correttamente con qualsiasi input che rispetti i limiti assegnati nel testo.

I programmi devono obbligatoriamente leggere i dati in ingresso da un file di input di nome assegnato e produrre i risultati su un file di output di nome assegnato. In particolare, questi file vanno aperti in C/C++ con le istruzioni:

`fr = fopen ( “input.txt”, “r”) ;`

`fw = fopen ( “output.txt”, “w”) ;`

e in Pascal con le istruzioni

`assign ( fr, ‘input.txt’ ) ; reset ( fr ) ;`

`assign (fw, ‘output.txt ‘) ; rewrite ( fw ) ;`

Il nome del file che contiene il programma deve essere esattamente il nome breve riportato nel testo del problema.

Il programma consegnato non deve interagire in alcun modo con l’utente, né stampare dati non richiesti, anche se nello sviluppo del programma è possibile utilizzare l’input/output da tastiera/video per eseguire debugging e testing.

Ne deriva che tutte le stampe a video e tutte le letture da tastiera devono essere assenti nella versione finale, pena la non valutazione del programma.

I file di input e di output devono risiedere nella stessa directory in cui risiede il programma sviluppato dal partecipante. Quindi essi devono essere indicati, nel codice sorgente, senza alcun pathname (nome di percorso).

I compilatori, il sistema operativo ed il software a disposizione nella sede di gara, nonché ulteriori informazioni tecniche, saranno reperibili sul sito web delle olimpiadi a partire dal 15 marzo 2010.

I partecipanti possono scegliere quali problemi risolvere e in quale ordine.

***5. Modalità di gara***

a) Per Sede Territoriale s’intende la scuola ove si effettua la prova di selezione. In funzione del numero dei partecipanti, alcune regioni hanno più sedi ed altre sono accorpate.

b) La selezione ha inizio per tutte le Sedi Territoriali alle ore 13.30 del 14 aprile 2010

c) I partecipanti sono identificati attraverso un documento di riconoscimento in corso di validità.

d) Ai partecipanti vengono consegnate le credenziali di accesso (username e password) al sistema di esami.

e) I partecipanti hanno a disposizione 30 minuti per prendere visione dell’ambiente di gara e per controllarne la corretta funzionalità.

f) Scaduto tale tempo, ai partecipanti viene dato accesso alle prove di esame, cioè viene abilitato il sistema di erogazione che rende disponibile il testo dei problemi tramite un browser. Ai partecipanti che ne facciano esplicita richiesta per motivate necessità, può essere fornita copia cartacea delle prove.

g) La durata della gara è di 3 ore dal momento in cui viene dato accesso alle prove di esame. Nessun partecipante può lasciare l’aula prima di 90 minuti dall’inizio della prova. Il testo può essere portato fuori dall’aula solo dopo il termine della prova.

h) Durante la prova non è possibile rivolgere alcuna domanda di chiarimento sul testo dei problemi e sul relativo svolgimento.

i) E' vietato consultare testi, manuali o appunti di qualsiasi genere, pena l’esclusione dalla prova. Analogamente è vietato utilizzare qualsiasi dispositivo elettronico al di fuori di quello espressamente messo a disposizione dall’organizzazione di gara. Le postazioni di lavoro disponibili per lo svolgimento della prova NON devono essere connesse ad Internet.

j) Al termine della prova, il partecipante deve consegnare gli elaborati utilizzando il sistema di sottomissione. Per il superamento della prova viene valutato l’eseguibile, ma l’archivio compresso inviato per la correzione deve contenere sia il sorgente che l’eseguibile di ciascun programma, pena la non valutazione.

k) I sorgenti e gli eseguibili devono avere il nome breve dell’esercizio svolto. Se, ad esempio, l’esercizio ha per nome nomebreve, il suo sorgente deve essere nomebreve.c oppure nomebreve.pas a seconda che sia in C/C++ o in Pascal. L'eseguibile è in ogni caso nomebreve.exe.

***6. Compilatori ufficiali della competizione***

I compilatori ufficiali sono il compilatore GNU per il C/C++ e il Free Pascal per il Pascal in ambiente Linux a 32 bit. E’ anche possibile utilizzare l’ambiente Windows. Poiché alcuni compilatori installati sotto Windows prevedono estensioni non standard, si consiglia in ogni caso di:

• utilizzare solo variabili intere a 32 bit (LongInt e Long);

• evitare l’uso di componenti non-standard, come la crt del Turbo Pascal o i file di intestazione, come conio.h del Turbo C;

• compilare in ogni caso con il compilatore ufficiale prima della consegna .

Il sito Web delle Olimpiadi contiene le istruzioni su come reperire i compilatori.

***7. Modalità di correzione ed assegnazione punteggi***

Per la valutazione della gara ogni programma viene eseguito su un insieme di casi di prova e viene valutato il numero q di casi per i quali si ottiene il risultato atteso. Al programma vengono quindi assegnati q*D punti (dove D è il coefficiente di difficoltà del problema).Il punteggio finale consiste nella somma dei punteggi ottenuti da ciascun programma.Lo stile di programmazione non ha alcun effetto sulla valutazione, così come il tempo necessario a risolvere ogni caso.

Contrariamente a quanto avveniva negli anni precedenti, non viene assegnato alcun punteggio per l’avvenuta compilazione.

***8. Classifiche ed ammissione alla gara di selezione nazionale, cioè alle Olimpiadi Italiane di Informatica***

Sono previste le seguenti classifiche:

a) Classifica regionale, stilata per ciascuna Regione in funzione del punteggio ottenuto dai singoli partecipanti.

b) Classifica generale, stilata fondendo ed ordinando le classifiche di tutte le Regioni.

A parità di punteggio, in qualsiasi classifica sono favoriti gli studenti di classe inferiore e, in caso di ulteriore parità, i più giovani.

Vengono ammessi alla gara di selezione nazionale, cioè alle Olimpiadi Italiane di Informatica dell’anno 2010:

a) i probabili Olimpici che ancora frequentano la scuola e sono nati dopo il 30 giugno dell’anno 1991;

b) il primo classificato di ogni Regione con punteggio eguale o maggiore al valore medio della classifica generale calcolato escludendo i punteggi nulli;

c) i migliori della classifica generale, esclusi i primi delle classifiche territoriali, fino ad un massimo di 80 partecipanti complessivi.

IL COMITATO OLIMPICO

Gennaio 2010

## Sedi

| **SEDE**      | **PROVINCE**                         |     | **ISTITUTO**                            | **COMUNE**    | **PROV.** | **REFERENTE**                  | **E-MAIL REFERENTE**                                                                  |
| ------------- | ------------------------------------ | --- | --------------------------------------- | ------------- | --------- | ------------------------------ | ------------------------------------------------------------------------------------- |
| **ASSEGNATE** | **ATLETI**                           |     |                                         |               |           |                                |                                                                                       |
| **ABR**       | AQ-CH-PE-TE                          | 21  | ITCG "G. Manthonè"                      | Pescara       | PE        | Prof.ssa Rossana D'Ignazio     | [digros@katamail.com](mailto:digros@katamail.com)                                     |
| **BAS**       | MT-PZ                                | 11  | L.S. "Federico II di Svevia"            | Melfi         | PZ        | Prof.ssa Teresa Caruso         | [teresa.caruso@istruzione.it](mailto:teresa.caruso@istruzione.it)                     |
| **CAL**       | CS-CZ-RC                             | 42  | ITIS "E. Scalfaro"                      | Catanzaro     | CZ        | Prof. Bruno Dattilo            | [bruno.dattilo@istruzione.it](mailto:bruno.dattilo@istruzione.it)                     |
| **CAM1**      | CE-BN-NA<br/>(Esclusa C. Stabia)     | 34  | Università di Napoli<br/>Sede di Agnano | Napoli        | NA        | Prof.re Lucio Pianese          | \-                                                                                    |
| **CAM2**      | AV-SA<br/>(Inclusa C. Stabia)        | 17  | ITI "B. Focaccia"                       | Salerno       | SA        | Prof. Agostino Clavelli        | [agostino.clavelli@istruzione.it](mailto:agostino.clavelli@istruzione.it)             |
| **EMI1**      | BO-FE                                | 28  | L.S "Enrico Fermi"                      | Bologna       | BO        | Prof.ssa Giorgeliana Carletto  | [gcarletto@email.it](mailto:gcarletto@email.it)                                       |
| **EMI2**      | FC-RN-RA                             | 25  | ITIS "Blaise Pascal"                    | Cesena        | FC        | Prof.ssa Vanna Zabberoni       | [vanna.zabberoni@itis-cesena.it](mailto:vanna.zabberoni@itis-cesena.it)               |
| **EMI3**      | MO-RE                                | 33  | I.T.G. "Blaise Pascal"                  | Reggio Emilia | RE        | Prof.ssa Barbara Cattani       | [barbara-cattani@libero.it](mailto:barbara-cattani@libero.it)                         |
| **EMI4**      | PR-PC                                | 14  | L.S. "Marconi"                          | Parma         | PR        | Prof.ssa Maria Botta           | [maria_botta@libero.it](mailto:maria_botta@libero.it)                                 |
| **FRI**       | GO-PN-TS-UD                          | 30  | ITI "Arturo Malignani"                  | Udine         | UD        | Prof.ssa Nicoletta Negrello    | [negrello@malignani.ud.it](mailto:negrello@malignani.ud.it)                           |
| **LAZ**       | FR-LT-RI-RM-VT                       | 45  | IIS Viale di Villa Pamphili, 86         | Roma          | RM        | Prof.ssa Clelia Somma          | [clelia.somma@istruzione.it](mailto:clelia.somma@istruzione.it)                       |
| **LIG**       | GE-IM-SP-SV                          | 16  | ITC "Carlo Rosselli"                    | Genova        | GE        | Prof.ssa Genzianella Foresti   | [geforest@tin.it](mailto:geforest@tin.it)                                             |
| **LOM1**      | LO-PV-MI<br/>(escluso Rho e Legnano) | 25  | IIS "Enrico Mattei"                     | San Donato M. | MI        | Prof. Luca Mazzei              | [lmazzei@libero.it](mailto:lmazzei@libero.it)                                         |
| **LOM2**      | CO-VA-Rho e Legnano                  | 18  | ITC "Enrico Tosi"                       | Busto Arsizio | VA        | Prof.ssa Silvia Tondo          | [silvia.tondo@itctosi.va.it](mailto:silvia.tondo@itctosi.va.it)                       |
| **LOM3**      | BG-LC-SO                             | 34  | ITI "P. Paleocapa"                      | Bergamo       | BG        | Prof.ssa Antonella Gualteroni  | [antgua@yahoo.com](mailto:antgua@yahoo.com)                                           |
| **LOM4**      | BS-CR-MN                             | 40  | ITIS "Benedetto Castelli"               | Brescia       | BS        | Prof. Alessandro Bugatti       | [alessandro.bugatti@istruzione.it](mailto:alessandro.bugatti@istruzione.it)           |
| **MAR**       | AN-AP-MC-PU-FM                       | 38  | IIS "V. Volterra"                       | Ancona        | AN        | Prof. Bruno Nossa              | [prb.nossa@istitutovolterraelia.it](mailto:prb.nossa@istitutovolterraelia.it)         |
| **MOL**       | CB-IS                                | 11  | L.S. "E. Majorana"                      | Isernia       | IS        | Prof. Stefano Sferra           | [s.sferra@tiscali.it](mailto:s.sferra@tiscali.it)                                     |
| **PIE1**      | AL-CN-TO                             | 32  | IIS "Baldessano-Roccati"                | Carmagnola    | TO        | Prof. Giovanni Spadafora       | [giovanni.spadafora@istruzione.it](mailto:giovanni.spadafora@istruzione.it)           |
| **PIE2**      | BI-NO-VB-VC                          | 13  | ITIS "Quintino Sella"                   | Biella        | BI        | Prof. Sandro Landorno          | [slandorno@itis.biella.it](mailto:slandorno@itis.biella.it)                           |
| **PUG1**      | BA                                   | 51  | ITCS "D. Romanazzi"                     | Bari          | BA        | Prof. Antonio De Carne         | [referente.pug1@libero.it](mailto:referente.pug1@libero.it)                           |
| **PUG2**      | BR-TA                                | 35  | ITI "E. Majorana"                       | Brindisi      | BR        | Prof. Giuseppe Cavallo         | [giohorse@tin.it](mailto:giohorse@tin.it)                                             |
| **PUG3**      | FG                                   | 11  | ITC "Blaise Pascal"                     | Foggia        | FG        | Prof. Lucio Antonio Tortorella | [lucioantonio.tortorella@istruzione.it](mailto:lucioantonio.tortorella@istruzione.it) |
| **PUG4**      | LE                                   | 36  | ITI "Enrico Mattei"                     | Maglie        | LE        | Prof. Nunzio Galati            | [galati@itismaglie.it](mailto:galati@itismaglie.it)                                   |
| **SAR**       | CA-NU-OR-SS                          | 13  | ITIS "Michele Giua"                     | Cagliari      | CA        | Prof.ssa Grazia Chiuchiolo     | [grazia.chiuchiolo@tiscali.it](mailto:grazia.chiuchiolo@tiscali.it)                   |
| **SIC1**      | EN-RG-SR                             | 10  | ITI "E. Fermi"                          | Siracusa      | SR        | Prof. Luciano Santo            | [sanluciano@tin.it](mailto:sanluciano@tin.it)                                         |
| **SIC2**      | AG-ME-PA-TP                          | 30  | ITI "Vittorio Emanuele III"             | Palermo       | PA        | Prof. Salvatore Truncali       | [truncali.s@libero.it](mailto:truncali.s@libero.it)                                   |
| **SIC3**      | CL-CT                                | 28  | ITI "Cannizzaro"                        | Catania       | CT        | Prof.ssa Cettina Allegra       | [cettina.allegra@tin.it](mailto:cettina.allegra@tin.it)                               |
| **TOS1**      | AR-FI-GR-PO-SI                       | 24  | ITI "T. Buzzi"                          | Prato         | PO        | Prof.ssa Grazia Biondi         | [g.biondi@itistulliobuzzi.it](mailto:g.biondi@itistulliobuzzi.it)                     |
| **TOS2**      | LI-LU-MS-PI-PT                       | 29  | ITI "Galileo Galilei"                   | Livorno       | LI        | Prof. Salvatore Speranza       | [salvatoresperanza@alice.it](mailto:salvatoresperanza@alice.it)                       |
| **TRE**       | BZ-TN                                | 29  | L.S. "G. Galilei"                       | Trento        | TN        | Prof.ssa Gabriella Armani      | [gabarm57@hotmail.com](mailto:gabarm57@hotmail.com)                                   |
| **UMB**       | PG-TR                                | 19  | LS "R. Donatelli"                       | Terni         | TR        | Prof.ssa Mara Massarucci       | [maramas@alice.it](mailto:maramas@alice.it)                                           |
| **VEN1**      | PD-RO-VE                             | 35  | ITI "Carlo Zuccante"                    | Venezia       | VE        | Prof. Carlo Salvagno           | [c_salvagno@tin.it](mailto:c_salvagno@tin.it)                                         |
| **VEN2**      | VR                                   | 23  | ITI "Guglielmo Marconi"                 | Verona        | VR        | Prof. Lorenzo De Carli         | [ldecarli@marconivr.it](mailto:ldecarli@marconivr.it)                                 |
| **VEN3**      | VI                                   | 22  | LS "Tron"                               | Schio         | VI        | Prof. Angelo Franco Catena     | [angelo.catena@tron.vi.it](mailto:angelo.catena@tron.vi.it)                           |
| **VEN4**      | BL-TV                                | 30  | IIS "Luigi Einaudi"                     | Montebelluna  | TV        | Prof.ssa Monica Bertuola       | [monica.bertuola@istruzione.it](mailto:monica.bertuola@istruzione.it)                 |

## Seminario Volterra - SIAF – 15/17 marzo 2010

**Allenamenti Olimpiadi italiane di informatica**

Seminario per docenti-referenti territoriali

Volterra – SIAF – 15/17 marzo 2010

Programma dei lavori



| **Data** | **Attività** | **Relatori** |
| -------- | ------------ | ------------ |
|Lunedì 15 marzo|||
|9.00|Apertura dei lavori e presentazione delle attività|Presidente OII <br> Marta Genovié de Vita|
|9.30|Presentazione esiti della selezione scolastica <br> Presentazione Correttore on line|Prof. Nello Scarabottolo <br> Università di Milano|
|10.30|Supporti per la preparazione degli studenti <br> Riflessioni e suggerimenti sulla strategia di gara|Prof. Giorgio Casadei <br> Università di Bologna|
|11.00|Coffee break||
|11.15|Prova in laboratorio del Correttore on line|Coordinano: <br> Prof.Nello Scarabottolo <br> Prof. Roberto Grossi <br> Tutor|
|12.30|Dibattito|Prof. Giorgio Casadei <br> Università di Bologna|
|13.00|Pranzo||
|15.00|Lectio magistralis <br> Nascere digitali: <br> verso un mutamento antropologico?|Prof. Giuseppe O. Longo <br> Università di Trieste|
|17.00|Coffee break||
|17.15|Dibattito||
|18.30|Riflessioni conclusive||
|19.00|Termine lavori||

<br>
<br>
<br>
<br>

| **Data** | **Attività** | **Relatori** |
| -------- | ------------ | ------------ |
|Martedì 16 marzo|||
|9.00|Approfondimenti disciplinari <br> Sezione di lavoro per gruppi sui seguenti temi <br> - Algoritmica <br> - Programmazione C/C++ <br> - Linux|Prof.  Roberto Grossi <br> Università di  Pisa <br> <br> Prof.  Nicola Galesi <br> Università di  Roma|
|11.00|Coffee break|Tutor|
|11.15|Approfondimenti disciplinari (segue)||
|13.00|Pranzo||
|15.00|Approfondimenti disciplinari (segue)||
|17.00|Coffee break||
|17.15|Approfondimenti disciplinari (segue)||
|19.00|Termine lavori||

<br>
<br>
<br>
<br>

| **Data** | **Attività** | **Relatori** |
| -------- | ------------ | ------------ |
|Mercoledì 17 marzo|||
|9.00|Lavori di gruppo: Elaborazione ipotesi di interventi <br> formativi per gli studenti|Coordinano: <br> Prof. Giorgio Casadei <br> Prof. Marta Genovié|<br> Prof.  Nicola Galesi <br> Prof. Nello Scarabottolo|
|11.00|Coffee break||
|11.15|Presentazione ipotesi di intervento||
|12.00|Dibattito||
|12.30|Conclusioni|Presidente OII <br> Marta Genovié de Vita|
|13.00|Pranzo||

<br>
<br>
<br>




**A questa iniziativa hanno partecipato 29 docenti:**

| Rossana       | D'Ignazio   | [digros@katamail.com](mailto:digros@katamail.com)                                         | ITCG "G. Manthonè"                      | Pescara       | PE  |
| ------------- | ----------- | ----------------------------------------------------------------------------------------- | --------------------------------------- | ------------- | --- |
| Teresa        | Caruso      | [teresa.caruso@istruzione.it](mailto:teresa.caruso@istruzione.it)                         | Liceo Scientifico Federico II di Svevia | Melfi         | PZ  |
| Agostino      | Clavelli    | [agostino.clavelli@istruzione.it](mailto:agostino.clavelli@istruzione.it)                 | ITIS "Focaccia"                         | Salerno       | SA  |
| Giorgeliana   | Carletto    | [gcarletto@email.it](mailto:gcarletto@email.it)                                           | Liceo Scientifico "Enrico Fermi"        | Bologna       | BO  |
| Vanna         | Zabberoni   | [vanna.zabberoni@itis-cesena.it](mailto:vanna.zabberoni@itis-cesena.it)                   | ITIS "Blaise Pascal"                    | Cesena        | FC  |
| Barbara       | Cattani     | barbara-cattani@libero.it                                                                 | ITG "Blaise Pascal"                     | Reggio Emilia | RE  |
| Giulio        | Angiani     | giulio.angiani@gmail.com                                                                  | ITG "Blaise Pascal"                     | Reggio Emilia | RE  |
| Nicoletta     | Negrello    | [negrello@malignani.ud.it](mailto:negrello@malignani.ud.it)                               | ITI "Arturo Malignani"                  | Udine         | UD  |
| Ezio          | Savasta     | [clelia.somma@istruzione.it](mailto:clelia.somma@istruzione.it)                           | IIS VIALE DI VILLA PAMPHILI, 86         | Roma          | RM  |
| Genzianella   | Foresti     | [geforest@tin.it](mailto:geforest@tin.it)                                                 | ITC "Carlo Rosselli"                    | Genova        | GE  |
| Cesare        | Bottaro     | cesare.bottaro@gmail.com                                                                  | ITIS "Paleocapa"                        | Bergamo       | BG  |
| Alessandro    | Bugatti     | [alessandro.bugatti@istruzione.it](mailto:alessandro.bugatti@istruzione.it)               | ITIS "Benedetto Castelli"               | Brescia       | BS  |
| Stefano       | Sferra      | s.sferra@tiscali.it                                                                       | Liceo Scientifico "E. Majorana"         | Isernia       | IS  |
| Sandro        | Landorno    | [slandorno@itis.biella.it](mailto:slandorno@itis.biella.it)                               | ITIS "Quintino Sella"                   | Biella        | BI  |
| Antonio       | De Carne    | [referente.pug1@libero.it](mailto:referente.pug1@libero.it)                               | ITCS "D. Romanazzi"                     | Bari          | BA  |
| Giuseppe      | Cavallo     | [giohorse@tin.it](mailto:giohorse@tin.it)                                                 | I.T.I. Majorana                         | Brindisi      | BR  |
| Lucio Antonio | Tortorella  | [lucioantonio.tortorella@istruzione.it](mailto:lucioantonio.tortorella@istruzione.it)     | ITC "Pascal"                            | Foggia        | FG  |
| Nunzio        | Galati      | [galati@itismaglie.it](mailto:galati@itismaglie.it)                                       | ITI "Mattei"                            | Maglie        | LE  |
| Grazia        | Chiuchiolo  | [grazia.chiuchiolo@tiscali.it](mailto:grazia.chiuchiolo@tiscali.it)                       | ITIS "Michele Giua"                     | Cagliari      | CA  |
| Salvatore     | Truncali    | [salvatore.truncali@istruzione.it](mailto:salvatore.truncali@istruzione.it)               | ITI "Vittorio Emanuele III"             | Palermo       | PA  |
| Andrea        | Mola        | [cettina.allegra@tin.it](mailto:cettina.allegra@tin.it)                                   | ITIS "Cannizzaro"                       | Catania       | CT  |
| Grazia        | Biondi      | g.biondi@itistulliobuzzi.it                                                               | ITI "Antonio Meucci"                    | Firenze       | FI  |
| Salvatore     | Speranza    | [salvatoresperanza@alice.it](mailto:salvatoresperanza@alice.it)                           | ITI "Galilei"                           | Livorno       | LI  |
| Gabriella     | Armani      | [gabarm57@hotmail.com](mailto:gabarm57@hotmail.com)                                       | Liceo Scientifico "G. Galilei"          | Trento        | TN  |
| Giuseppe      | Corinaldesi | [prg.corinaldesi@istitutovolterraelia.it](mailto:prg.corinaldesi@istitutovolterraelia.it) | ITIS VOLTERRA                           | Ancona        | AN  |
| Mara          | Massarucci  | [maramas@alice.it](mailto:maramas@alice.it)                                               | Liceo Scientifico "R. Donatelli"        | Terni         | TR  |
| Carlo         | Salvagno    | [c_salvagno@tin.it](mailto:c_salvagno@tin.it)                                             | ITI "Carlo Zuccante"                    | Mestre        | VE  |
| Lorenzo       | De Carli    | [ldecarli@marconivr.it](mailto:ldecarli@marconivr.it)                                     | ITI "Guglielmo Marconi"                 | Verona        | VR  |
| A.Franco      | Catena      | [angelo.catena@tron.vi.it](mailto:angelo.catena@tron.vi.it)                               | Liceo Scientifico "N. Tron"             | Schio         | VI  |

## Atleti ammessi

Selezione regionale - Atleti ammessi

| [ABR](ioi/documents/edition/2010/atletiAmmessiRegionali/ABR.pdf) | [BAS](ioi/documents/edition/2010/atletiAmmessiRegionali/BAS.pdf) | [CAL](ioi/documents/edition/2010/atletiAmmessiRegionali/CAL.pdf) | [CAM](ioi/documents/edition/2010/atletiAmmessiRegionali/CAM.pdf) |
| ---------------------------------------------------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------------- |
| [EMI](ioi/documents/edition/2010/atletiAmmessiRegionali/EMI.pdf) | [FRI](ioi/documents/edition/2010/atletiAmmessiRegionali/FRI.pdf) | [LAZ](ioi/documents/edition/2010/atletiAmmessiRegionali/LAZ.pdf) | [LIG](ioi/documents/edition/2010/atletiAmmessiRegionali/LIG.pdf) |
| [LOM](ioi/documents/edition/2010/atletiAmmessiRegionali/LOM.pdf) | [MAR](ioi/documents/edition/2010/atletiAmmessiRegionali/MAR.pdf) | [MOL](ioi/documents/edition/2010/atletiAmmessiRegionali/MAR.pdf) | [PIE](ioi/documents/edition/2010/atletiAmmessiRegionali/MOL.pdf) |
| [PUG](ioi/documents/edition/2010/atletiAmmessiRegionali/PUG.pdf) | [SAR](ioi/documents/edition/2010/atletiAmmessiRegionali/SAR.pdf) | [SIC](ioi/documents/edition/2010/atletiAmmessiRegionali/SIC.pdf) | [TOS](ioi/documents/edition/2010/atletiAmmessiRegionali/TOS.pdf) |
| [TRE](ioi/documents/edition/2010/atletiAmmessiRegionali/TRE.pdf) | [UMB](ioi/documents/edition/2010/atletiAmmessiRegionali/UMB.pdf) | [VEN](ioi/documents/edition/2010/atletiAmmessiRegionali/VEN.pdf) | \-                                                               |

## Prove assegnate

Sono disponibili le prove assegnate alla Selezione Territoriale del 14 Aprile 2010

[Quasipal](/oldsite/105/quasipal.pdf)

[Sbarramento](/oldsite/105/sbarramento.pdf)

[Tamburello](/oldsite/105/tamburello.pdf)

## Formazione

## Iniziative di formazione per la gara nazionale

In questa pagina sono segnalate le iniziative di formazione organizzate a livello territoriale.

* 22 maggio 2010 - ITIS B. Castelli, Brescia:  [Algoritmi in Festa](http://81.208.32.83:8080/ioi/files/algoritmi%20in%20festa.pdf) ([Locandina](http://81.208.32.83:8080/ioi/files/Locandina_bugatti.pdf))
* 22 maggio 2010 - Liceo Scient. E. Fermi, Bologna:  [Olimpiadi di Informatica a Squadre 2010/2011 - I Edizione](http://81.208.32.83:8080/ioi/files/Rel__Prof__Carletto-Gare_reg_Emilia.pdf)