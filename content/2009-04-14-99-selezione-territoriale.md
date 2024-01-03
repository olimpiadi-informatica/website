+++
draft = false
title = "Selezione territoriale 2008-2009"
date = 2009-04-14T11:50:29Z
updated = "2016-10-20T13:24:01"

[taxonomies]
contest = [ "regional",]
edition = [ "2008-2009",]

[extra]
override_updated = "2016-10-20T13:24:01"
+++
La selezione territoriale delle Olimpiadi di Informatica si è svolta il 2 aprile 2009.

## Regolamento

**INTERNATIONAL OLYMPIAD IN INFORMATICS**

**Olimpiadi Italiane di Informatica**

**Regolamento della gara di selezione territoriale del 14 aprile 2010**

**(IOI 2011)**

<div style="text-align: center;">

</div>

<div style="text-align: center;">

</div>
**1. Ammissione alla gara di selezione**

Per partecipare lo studente deve:

• avere superato la selezione scolastica,

• essere disponibile, qualora superi la selezione territoriale, a partecipare alla gara nazionale ed a frequentare i corsi di formazione che si terranno prima della prova internazionale,

• essere disponibile, qualora superi l’ultima selezione, a recarsi all’estero nel periodo della gara internazionale con gli accompagnatori designati.

**_2. Caratteristiche della gara_**

Obiettivo della gara è verificare le capacità dei partecipanti nel risolvere problemi mediante la scrittura di programmi secondo lo stile delle Olimpiadi Internazionali di Informatica. I problemi proposti sono simili a quelli proposti in sede internazionale, anche se mediamente più semplici.

**3. Descrizione dei problemi**

La gara consiste nella soluzione di tre problemi, per ognuno dei quali sono fornite le seguenti informazioni:

• nome breve, che individua il problema;

• coefficiente di difficoltà D (crescente con la difficoltà)

• descrizione del problema

• limiti e/o condizioni dei dati di ingresso

• esemplificazione per mezzo di uno o più casi di prova

• eventuali note.

**4. Soluzione dei problemi**

I problemi devono essere risolti tramite programmi scritti in linguaggio C/C++ o in linguaggio Pascal; tali programmi devono funzionare correttamente con qualsiasi input che rispetti i limiti assegnati nel testo.

I programmi devono obbligatoriamente leggere i dati in ingresso da un file di input di nome assegnato e produrre i risultati su un file di output di nome assegnato. In particolare, questi file vanno aperti in C/C++ con le istruzioni:

fr = fopen ( “input.txt”, “r”) ;

fw = fopen ( “output.txt”, “w”) ;

e in Pascal con le istruzioni

assign ( fr, ‘input.txt’ ) ; reset ( fr ) ;

assign (fw, ‘output.txt ‘) ; rewrite ( fw ) ;

Il nome del file che contiene il programma deve essere esattamente il nome breve riportato nel testo del problema.

Il programma consegnato non deve interagire in alcun modo con l’utente, né stampare dati non richiesti, anche se nello sviluppo del programma è possibile utilizzare l’input/output da tastiera/video per eseguire debugging e testing.

Ne deriva che tutte le stampe a video e tutte le letture da tastiera devono essere assenti nella versione finale, pena la non valutazione del programma.

I file di input e di output devono risiedere nella stessa directory in cui risiede il programma sviluppato dal partecipante. Quindi essi devono essere indicati, nel codice sorgente, senza alcun pathname (nome di percorso).

I compilatori, il sistema operativo ed il software a disposizione nella sede di gara, nonché ulteriori informazioni tecniche, saranno reperibili sul sito web delle olimpiadi a partire dal 15 marzo 2010.

I partecipanti possono scegliere quali problemi risolvere e in quale ordine.

**5. Modalità di gara**

a)Per Sede Territoriale s’intende la scuola ove si effettua la prova di selezione. In funzione del numero dei partecipanti, alcune regioni hanno più sedi ed altre sono accorpate.

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

**6. Compilatori ufficiali della competizione**

I compilatori ufficiali sono il compilatore GNU per il C/C++ e il Free Pascal per il Pascal in ambiente Linux a 32 bit. E’ anche possibile utilizzare l’ambiente Windows. Poiché alcuni compilatori installati sotto Windows prevedono estensioni non standard, si consiglia in ogni caso di:

• utilizzare solo variabili intere a 32 bit (LongInt e Long);

• evitare l’uso di componenti non-standard, come la crt del Turbo Pascal o i file di intestazione, come conio.h del Turbo C;

• compilare in ogni caso con il compilatore ufficiale prima della consegna .

Il sito Web delle Olimpiadi contiene le istruzioni su come reperire i compilatori.

**7. Modalità di correzione ed assegnazione punteggi**

Per la valutazione della gara ogni programma viene eseguito su un insieme di casi di prova e viene valutato il numero q di casi per i quali si ottiene il risultato atteso. Al programma vengono quindi assegnati q\*D punti (dove D è il coefficiente di difficoltà del problema).Il punteggio finale consiste nella somma dei punteggi ottenuti da ciascun programma.Lo stile di programmazione non ha alcun effetto sulla valutazione, così come il tempo necessario a risolvere ogni caso.

Contrariamente a quanto avveniva negli anni precedenti, non viene assegnato alcun punteggio per l’avvenuta compilazione.

**8. Classifiche ed ammissione alla gara di selezione nazionale, cioè alle Olimpiadi Italiane di Informatica**

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

| **Elenco delle Sedi Territoriali**<br/>
|:---: |
| SEDE | PROVINCE <br/> ASSEGNATE | ISTITUTO | INDIRIZZO | COMUNE | PROV. | REFERENTE | E-MAIL REFERENTE |
| ABR | AQ - CH - PE - TE | ITCG "G. Manthonè" | Via Tiburtina, 202 | Pescara | PE | Prof.ssa Rossana D'Ignazio | [digros@katamail.com](mailto:digros@katamail.com) |
| BAS | MT - PZ | L.S. "Federico II di Svevia" | Via Verdi, 1 | Melfi | PZ | Prof.ssa Teresa Caruso | [teresa.caruso@istruzione.it](mailto:teresa.caruso@istruzione.it) |
| CAL | CS - CZ - RC | ITIS "A. Panella" | Via Emilio Cuzzocrea, 22 | Reggio Calabria | RC | Prof. Saverio Spinelli | [savspi@libero.it](mailto:savspi@libero.it) |
| CAM1 | BN-CE-NA | ITI "G. Ferraris" | Via A. Labriola, Lotto 2G | Napoli | NA | Prof. Gaetano Calandrelli | [calandrelli@infinito.it](mailto:calandrelli@infinito.it) |
| CAM2 | AV-SA | ITI "B. Focaccia" | Via Monticelli, 1 | Salerno | SA | Prof. Agostino Clavelli | [agostino.clavelli@istruzione.it](mailto:agostino.clavelli@istruzione.it) |
| EMI1 | BO-FE | L.S "Enrico Fermi" | Via Mazzini, 172/2 | Bologna | BO | Prof.ssa Giorgeliana Carletto | [gcarletto@email.it](mailto:gcarletto@email.it) |
| EMI2 | FC-RN-RA | ITIS "Blaise Pascal" | Piazzale Cino Macrelli, 100 | Cesena | FC | Prof.ssa Vanna Zabberoni | [vanna.zabberoni@itis-cesena.it](mailto:vanna.zabberoni@itis-cesena.it) |
| EMI3 | MO-RE | I.T.G. "Blaise Pascal" | Via Makallé, 12 | Reggio Emilia | RE | Prof.ssa Annamaria Culzoni | [mariaculzoni@libero.it](mailto:mariaculzoni@libero.it) |
| EMI4 | PR-PC | L.S. "Marconi" | Via Costituente, 2 | Parma | PR | Prof.ssa Maria Botta | [maria_botta@libero.it](mailto:maria_botta@libero.it) |
| FRI | GO - PN - TS - UD | ITI "Arturo Malignani" | Viale Leonardo da Vinci, 10 | Udine | UD | Prof. Giuseppe Nobile | [nobile@malignani.ud.it](mailto:nobile@malignani.ud.it) |
| LAZ1 | RM Città | ITC "Federico Caffè" | Via di Villa Pamphili | Roma | RM | Prof. Aldo Capretti | [aldcpr@tin.it](mailto:aldcpr@tin.it) |
| LAZ2 | FR-LT-RI-VT<br/> RM prov | ITIS "Galileo Galilei" | Via Conte Verde, 51 | Roma | RM | Prof. Antonio Mastacchi | [a.mastacchi@tin.it](mailto:a.mastacchi@tin.it) |
| LIG | GE - IM - SP - SV | ITC "Carlo Rosselli" | Via Giotto, 10 | Genova | GE | Prof.ssa Genzianella Foresti | [geforest@tin.it](mailto:geforest@tin.it) |
| LOM1 | LO-PV<br/> MI (no Castano, Rho e Legnano | IIS "Enrico Mattei" | Via Martiri di Cefalonia, 46 | San DonatoMilanese | MI | Prof. Luca Mazzei | [lmazzei@libero.it](mailto:lmazzei@libero.it) |
| LOM2 | CO-VA-MI <br/> (solo Castano, Rho e Legnano) | ITC "Enrico Tosi" | Viale Stelvio, 173 | Busto Arsizio | VA | Prof.ssa Silvia Tondo | [silvia.tondo@itctosi.va.it](mailto:silvia.tondo@itctosi.va.it) |
| LOM3 | BG-LC-SO | ITI "P. Paleocapa" | Via M. Gavazzeni, 29 | Bergamo | BG | Prof.ssa Antonella Gualteroni | [antgua@yahoo.com](mailto:antgua@yahoo.com) |
| LOM4 | BS - CR - MN | ITIS "Benedetto Castelli" | Via A. Cantore, 9 | Brescia | BS | Prof. Alessandro Bugatti | [alessandro.bugatti@istruzione.it](mailto:alessandro.bugatti@istruzione.it) |
| MAR | AN - AP - MC - PU | ITI "V. Volterra" | Via Esino, 36 | Ancona | AN | Prof. Bruno Nossa | [prb.nossa@itis.an.it](mailto:prb.nossa@itis.an.it) |
| MOL | CB - IS | L.S. "E. Majorana" | C.so Risorgimento | Isernia | IS | Prof. Stefano Sferra | [s.sferra@tiscali.it](mailto:s.sferra@tiscali.it) |
| PIE1 | AL-AT-CN-TO | IIS "Baldessano-Roccati" | Viale Garibaldi, 7 | Carmagnola | TO | Prof. Giovanni Spadafora | [giovanni.spadafora@istruzione.it](mailto:giovanni.spadafora@istruzione.it) |
| PIE2 | BI-NO-VB-VC | ITIS "Quintino Sella" | Via Fratelli Rosselli, 2 | Biella | BI | Prof. Sandro Landorno | [slandorno@itis.biella.it](mailto:slandorno@itis.biella.it) |
| PUG1 | BA | ITCS "D. Romanazzi" | Via Celso Ulpiani, 6A | Bari | BA | Prof. Antonio De Carne | [antonio.decarne@istruzione.it](mailto:antonio.decarne@istruzione.it) |
| PUG2 | BR-TA | LS "Oria" | Via Ada Negri, 1 | Oria | BR | Prof. Carmelo Padula | [carmepadula@libero.it](mailto:carmepadula@libero.it) |
| PUG3 | FG | ITC "Blaise Pascal" | Via Napoli Km. 0,7 | Foggia | FG | Prof. Lucio Antonio Tortorella | [lucioantonio.tortorella@istruzione.it](mailto:lucioantonio.tortorella@istruzione.it) |
| PUG4 | LE | ITI "Enrico Mattei" | Via Nicola Ferramosca, 82 | Maglie | LE | Prof. Nunzio Galati | [nunzio.galati@istruzione.it](mailto:nunzio.galati@istruzione.it) |
| SAR | CA - NU - OR - SS | ITIS "Michele Giua" | Via Montecassino | Cagliari | CA | Prof.ssa Grazia Chiuchiolo | [grazia.chiuchiolo@tiscali.it](mailto:grazia.chiuchiolo@tiscali.it) |
| SIC1 | CL-EN-RG-SR | ITI "E. Fermi" | Via Torino 137 | Siracusa | SR | Prof. Luciano Santo | [sanluciano@tin.it](mailto:sanluciano@tin.it) |
| SIC2 | AG-ME-PA-TP | ITI "Vittorio Emanuele III" | Via Duca della Verdura, 48 | Palermo | PA | Prof. Salvatore Truncali | [truncali.s@libero.it](mailto:truncali.s@libero.it) |
| SIC3 | CT | ITI "Cannizzaro" | Via Carlo Pisacane, 1 | Catania | CT | Prof.ssa Cettina Allegra | [cettina.allegra@tin.it](mailto:cettina.allegra@tin.it) |
| TOS1 | AR-FI-PO-PT-SI | ITI "Antonio Meucci" | Via del Filarete, 17 | Firenze | FI | Prof. Stefano Bernardi | [s.bernardi@tin.it](mailto:s.bernardi@tin.it) |
| TOS2 | GR-LI-LU-MS-PI | ITI "Galileo Galilei" | Via Galilei, 68 | Livorno | LI | Prof. Salvatore Speranza | [salvatoresperanza@alice.it](mailto:salvatoresperanza@alice.it) |
| TRE | BZ - TN | L.S. "G. Galilei" | Viale Bolognini, 88 | Trento | TN | Prof.ssa Gabriella Armani | [gabarm57@hotmail.com](mailto:gabarm57@hotmail.com) |
| UMB | PG - TR | LS "R. Donatelli" | Via della Vittoria, 35 | Terni | TR | Prof.ssa Mara Massarucci | [maramas@alice.it](mailto:maramas@alice.it) |
| VEN1 | VE | ITI "Carlo Zuccante" | Via Baglioni, 22 | Venezia | VE | Prof. Carlo Salvagno | [c_salvagno@tin.it](mailto:c_salvagno@tin.it) |
| VEN2 | VR | ITI "Guglielmo Marconi" | Piazzale R. Guardini, 1 | Verona | VR | Prof. Lorenzo De Carli | [ldecarli@marconivr.it](mailto:ldecarli@marconivr.it) |
| VEN3 | VI | LS "Tron" | Via Luzio, 4 | Schio | VI | Prof. Angelo Franco Catena | [angelo.catena@tron.vi.it](mailto:angelo.catena@tron.vi.it) |
| VEN4 | BL-TV | IIS "Luigi Einaudi" | Via J. Sansovino, 6 | Montebelluna | TV | Prof.ssa Monica Bertuola | [monica.bertuola@istruzione.it](mailto:monica.bertuola@istruzione.it) |
| VEN5 | PD-RO | ITIS "Francesco Severi" | Via Pettinati, 46 | Padova | PD | Prof.ssa Maria Meneghini | [maria.meneghini@istruzione.it](mailto:maria.meneghini@istruzione.it) | | | |

## Atleti ammessi

Selezione regionale - Atleti ammessi

|  [ABR](/oldsite/99/ABR.pdf)  |  [BAS](/oldsite/99/BAS.pdf)  |  [CAL](/oldsite/99/CAL.pdf)  | [CAM1](/oldsite/99/CAM1.pdf) | [CAM2](/oldsite/99/CAM2.pdf) |
| :--------------------------: | :------------------------: | :------------------------: | :------------------------: | :------------------------: |
| [EMI1](/oldsite/99/EMI1.pdf) | [EMI2](/oldsite/99/EMI2.pdf) | [EMI3](/oldsite/99/EMI3.pdf) | [EMI4](/oldsite/99/EMI4.pdf) |  [FRI](/oldsite/99/FRI.pdf)  |
| [LAZ1](/oldsite/99/LAZ1.pdf) | [LAZ2](/oldsite/99/LAZ2.pdf) |  [LIG](/oldsite/99/LIG.pdf)  | [LOM1](/oldsite/99/LOM1.pdf) | [LOM2](/oldsite/99/LOM2.pdf) |
| [LOM3](/oldsite/99/LOM3.pdf) | [LOM4](/oldsite/99/LOM4.pdf) |  [MAR](/oldsite/99/MAR.pdf)  |  [MOL](/oldsite/99/MOL.pdf)  | [PIE1](/oldsite/99/PIE1.pdf) |
| [PIE2](/oldsite/99/PIE2.pdf) | [PUG1](/oldsite/99/PUG1.pdf) | [PUG2](/oldsite/99/PUG2.pdf) | [PUG3](/oldsite/99/PUG3.pdf) | [PUG4](/oldsite/99/PUG4.pdf) |
|  [SAR](/oldsite/99/SAR.pdf)  | [SIC1](/oldsite/99/SIC1.pdf) | [SIC2](/oldsite/99/SIC2.pdf) | [SIC3](/oldsite/99/SIC3.pdf) | [TOS1](/oldsite/99/TOS1.pdf) |
| [TOS2](/oldsite/99/TOS2.pdf) |  [TRE](/oldsite/99/TRE.pdf)  |  [UMB](/oldsite/99/UMB.pdf)  | [VEN1](/oldsite/99/VEN1.pdf) | [VEN2](/oldsite/99/VEN2.pdf) |
| [VEN3](/oldsite/99/VEN3.pdf) | [VEN4](/oldsite/99/VEN4.pdf) | [VEN5](/oldsite/99/VEN5.pdf) |             -              |             -              |

## Prove assegnate

Prove assegnate alla Selezione Territoriale del 02.04.2009

[Depurazione dell'acqua](/oldsite/99/depura.pdf)

[Essenza per profumi<br/>](/oldsite/99/essenza.pdf)

[Treno di container<br/>](/oldsite/99/treno.pdf)

## Formazione

## Iniziative di formazione per la gara nazionale

In questa pagina sono segnalate le iniziative di formazione organizzate a livello territoriale.

- 22 maggio 2010 - ITIS B. Castelli, Brescia:  [Algoritmi in Festa](http://81.208.32.83:8080/ioi/files/algoritmi%20in%20festa.pdf) ([Locandina](http://81.208.32.83:8080/ioi/files/Locandina_bugatti.pdf))
- 22 maggio 2010 - Liceo Scient. E. Fermi, Bologna:  [Olimpiadi di Informatica a Squadre 2010/2011 - I Edizione](http://81.208.32.83:8080/ioi/files/Rel__Prof__Carletto-Gare_reg_Emilia.pdf)
