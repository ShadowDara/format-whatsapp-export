// A very long Test Message for a Benchmark

#![feature(test)]

extern crate test;

use test::Bencher;

use msg_parser::{self, parsetxt};

const TXT: &str = r#"
11.10.23, 15:44 - Nachrichten und Anrufe sind Ende-zu-Ende-verschlüsselt. Nur Personen in diesem Chat können sie lesen, anhören oder teilen. Mehr erfahren
23.11.23, 22:20 - Nachrichten und Anrufe sind Ende-zu-Ende-verschlüsselt. Nur Personen in diesem Chat können sie lesen, anhören oder teilen. Mehr erfahren
08.05.24, 12:14 - sammy: Wer ist eig dein Partner?
08.05.24, 13:19 - Ole: Cameron
23.07.24, 08:07 - sammy: hey, du hast doch Informatik LK gewählt oder? Weißt du schon irgendwas, weil das ja nächstes Jahr nicht hier an der Schule stattfindet.
23.07.24, 12:48 - Ole: Später um 14 uhr ist ein treffen ich glaub in pc raum1
23.07.24, 12:51 - sammy: Ok danke
11.09.24, 16:20 - sammy: Hast jzt zug bekommen?
11.09.24, 16:56 - Ole: Ja hab ich, aber trotzdem danke für dss angebot
26.03.25, 18:35 - Ole: hey, du hast doch vorhin in inf am pc dasselbe eingetragen was herr lautebach vorgemacht hat, was unsere hausaufgabe ist, oder? könntest du mir das bitte schicken?
26.03.25, 19:02 - sammy: ‎Halbradierer.mod (Datei angehängt)
Halbradierer.mod
26.03.25, 19:02 - sammy: jup hier
26.03.25, 19:02 - sammy: die Beschreibung hab ich aber nicht ganz abgeschrieben
26.03.25, 19:03 - sammy: also das ist das zwischen Input und output
26.03.25, 19:03 - sammy: du must die Datei in den modules ordner machen
26.03.25, 19:06 - sammy: oder wolltest du mich nur fragen was wir überhaupt machen sollen? <Diese Nachricht wurde bearbeitet.>
26.03.25, 19:33 - Ole: alles klar, dankeschön
26.03.25, 19:34 - Ole: ne das hab ich mitbekommen, aber ich hab das nicht alles aufgeschrieben
30.03.25, 13:25 - Ole: wie hast du das gemacht? wir sollten doch so einen additionsrechner bauen oder?
30.03.25, 13:37 - sammy: ‎IMG-20250330-WA0001.jpg (Datei angehängt)
30.03.25, 13:37 - sammy: geh auf das ordner Symbol und öffne dann die Datei, dann siehst du den inhalt
30.03.25, 13:37 - sammy: ja, also ich hab den so gemacht, genauso wie er
30.03.25, 16:31 - Ole: Achso, dankeschön
02.04.25, 15:12 - sammy: <Medien ausgeschlossen>
4Bit Adiere.mod
06.04.25, 17:03 - Ole: mein negierer funktioniert irgw nicht, könntest du mir bitte mal deinen schicken?
06.04.25, 18:02 - sammy: jo, mach ich
06.04.25, 18:03 - sammy: ‎IMG-20250406-WA0002.jpg (Datei angehängt)
also ist der 4bit negierer mit + 1 für genau minuszahlen
06.04.25, 18:03 - sammy: <Medien ausgeschlossen>
4Bit Negierer.mod
06.04.25, 18:36 - Ole: Alles klar, danke
06.04.25, 20:11 - Ole: der geht bei mir nicht, ich glaub ich hab den volladierer falsch. könntest du mir evtl deinen mal schicken, wenn das keine umstände macht?
06.04.25, 20:12 - Ole: am 4 bit addierer hast du nichts mehr geändert seit du ihn mir geschickt hast oder?
06.04.25, 22:51 - Ole: jetzt gehts, brauchst nichts schicken
06.04.25, 23:19 - Ole: hast du alle inf hausaufgaben gemacht?
06.04.25, 23:19 - sammy: Jup
06.04.25, 23:20 - Ole: ‎IMG-20250406-WA0003.jpg (Datei angehängt)
das ist mein substrahierer. ist falsch oder?
06.04.25, 23:22 - sammy: Der is sieht richtig aus, hast du bei den negierer +1 gemacht?
06.04.25, 23:22 - Ole: ja
06.04.25, 23:22 - sammy: Aber es kommt wahrscheinlich Quatsch raus weil du 4 - 7 versuchst
06.04.25, 23:22 - sammy: Tausch das mal
06.04.25, 23:22 - sammy: Also bei den Knöpfen
06.04.25, 23:22 - Ole: ach so, ok
06.04.25, 23:23 - sammy: Also nur drücken fals du verstehst was ich meine
06.04.25, 23:23 - Ole: ja, ich versuchs mal
06.04.25, 23:24 - Ole: soll 0011 rauskommen?
06.04.25, 23:24 - sammy: Das wäre 3
06.04.25, 23:24 - sammy: Hört sich gut an
06.04.25, 23:24 - Ole: ok, vielen dank
07.04.25, 11:26 - sammy: ‎4Bit Subtrahierrer.mod (Datei angehängt)
4Bit Subtrahierrer.mod
07.04.25, 11:26 - sammy: ‎4Bit Adiere.mod (Datei angehängt)
4Bit Adiere.mod
07.04.25, 11:26 - sammy: ‎4Bit Negierer.mod (Datei angehängt)
4Bit Negierer.mod
07.04.25, 11:26 - sammy: ‎4bit-adierer-subtrahier.mod (Datei angehängt)
4bit-adierer-subtrahier.mod
07.04.25, 11:26 - sammy: ‎HA SEN.mod (Datei angehängt)
HA SEN.mod
07.04.25, 11:26 - sammy: ‎Halbradierer.mod (Datei angehängt)
Halbradierer.mod
07.04.25, 11:26 - sammy: ‎4plexer.mod (Datei angehängt)
4plexer.mod
07.04.25, 11:26 - sammy: ‎Multiplexer.mod (Datei angehängt)
Multiplexer.mod
07.04.25, 11:26 - sammy: ‎Volladierer.mod (Datei angehängt)
Volladierer.mod
07.04.25, 11:26 - sammy: ‎VA SEN.mod (Datei angehängt)
VA SEN.mod
07.04.25, 13:51 - sammy: Hast du inf richtig aufgeschrieben, also ha, wenn ja schick mal bitte
07.04.25, 16:03 - Ole: ‎IMG-20250407-WA0000.jpg (Datei angehängt)
Sag falls du was niczt lesen kannst
07.04.25, 18:32 - sammy: Danke
07.04.25, 18:40 - Ole: Kein problem
08.04.25, 19:18 - sammy: Äh was steht da ganz am ende und was nimmst du so als Rechenbeispiele? Sowas wie 1+1?
08.04.25, 20:21 - Ole: da steht man soll über die gleichheitszeichen schreiben welche rechenregel man angewandt hat
08.04.25, 20:23 - Ole: ich glaube als beispiele soll man was aus dem alltag nehmen oder so. als beispiel für irgendwas meinte er letztes mal das jdm nur die klausur zurückbekommt wenn beide bedingungen erfüllt sind: klausur geschrieben, klausur korrigiert. wenn nur eines zutrifft, geht das nicht. wir brauchen aber nicht bei allen ein beispiel angeben
08.04.25, 21:07 - sammy: Ah ok checke
08.04.25, 21:07 - sammy: Danke
08.04.25, 22:22 - Ole: wie hast du den term vereinfacht?
08.04.25, 22:56 - sammy: Ich konnte es auch nur auf zwei vereinfachen mit sowas wie a(s^s~)b und dann noch einen zweiten term, ich glaub so as~(b^b~) aber auch nicht sicher, ich könnte es dir morgen in der Schule zeigen <Diese Nachricht wurde bearbeitet.>
08.04.25, 22:58 - Ole: Ok danke
08.04.25, 22:58 - Ole: Ja morgen wär gut
09.04.25, 12:43 - sammy: ‎IMG-20250409-WA0005.jpg (Datei angehängt)
04.05.25, 13:48 - Ole: könntest du mir bitte schicken, wie du dieses speichersystem in inf gemacht hast?
04.05.25, 14:32 - sammy: Jo, war sonst eig noch was auf? <Diese Nachricht wurde bearbeitet.>
04.05.25, 14:33 - sammy: ‎2 bit controrl.mod (Datei angehängt)
2 bit controrl.mod
04.05.25, 14:33 - sammy: ‎d latch.mod (Datei angehängt)
d latch.mod
04.05.25, 14:33 - sammy: ‎2 bit minibytes.mod (Datei angehängt)
2 bit minibytes.mod
04.05.25, 14:33 - sammy: ‎rs nor latch.mod (Datei angehängt)
rs nor latch.mod
04.05.25, 14:34 - sammy: <Medien ausgeschlossen>
change_file_extension.ps1
04.05.25, 15:26 - Ole: ok, dankeschön! wir haben glaub sonst nichts auf, hab mir zumindest nichts aufgeschrieben
09.05.25, 11:47 - sammy: hey, was war eig in inf auf? Kannst du das bitte schicken?
09.05.25, 11:49 - Ole: wir sollten auf dem AB von mittwoch phase 9 + 10 fertig machen. und wir sollten das hexadezimalsystem üben, auch die umrechnung zu binär, weil wir das ja brauchen für das neue programm. aber so wie ich das verstanden hab sind beide aufgaben erst auf übernächste woche auf
09.05.25, 11:49 - sammy: ok danke
09.05.25, 11:49 - Ole: gern
22.05.25, 11:19 - Ole: https://prezi.com/yst2_retq0qs/wie-adolf-hitler-das-wort-legal-in-den-mund-nimmt/
22.05.25, 11:39 - sammy: danke
28.05.25, 12:18 - sammy: // Filtered on Montag 2025-Februar-24 at 11:23:33 by Tippfilter 0.5-rc4-3-g3f1bba0 (development or dirty build)

import greenfoot._

// scalac -cp .:lib/greenfoot.jar *.scala

abstract class FahrzeugS( private var vMax: Int ) extends Actor:
   private var vMom: Int = 0

   def fahre1s(): Unit =
      move(vMom)

   def beschleunige(deltaV: Int): Unit =
      vMom += deltaV
      vMom min vMax

   def getvMom = vMom

   def getvMax = vMax

   override def act(): Unit = 
      fahre1s()


class FahrradS(vMax: Int) extends FahrzeugS(vMax)


abstract class KfzS(val tankKapz: Double, val vMax: Int, val verbrauch: Double) extends FahrzeugS(vMax):
   private val tank_min: Double = 0;
   private var tankinhalt: Double = 0
   private var vMom = super.getvMom

   def tanke(menge: Int): Unit =
      if ((tankinhalt + menge) < tankKapz) /* && (tankinhalt > tank_min) */ then
         tankinhalt += menge //ToDO: Max –imum Minimum

   override def fahre1s(): Unit =
      if (tankinhalt > (verbrauch * vMom)) then
         super.fahre1s()
         verbraucheTank()
         //Tankinhalt anpassen

   def verbraucheTank(): Unit =
      tankinhalt -= (verbrauch * vMom)

// LKW
class LKWS extends KfzS(200.0, 80, 0.5):
   private var laderampeOffen = false
   protected var beladung: Ladegut_S = null
   private var vMom = super.getvMom

   def istBeladen = 
      beladung != null

   def istLaderampeOffen = laderampeOffen

    /** Öffnet die Laderampe, aber nur wenn das Fahrzeug steht. */
   def oeffneLaderampe(): Unit =
      if (vMom == 0) && (laderampeOffen == false) then
         laderampeOffen = true
         this.setImage(new GreenfootImage("lasterLadeklappeOffen.png"))


   /** Schließt die Laderampe     */
   def schliesseLaderampe(): Unit =
      if laderampeOffen then
         laderampeOffen = false
         this.setImage(new GreenfootImage("laster.png"))


    /** Verändert die Momentangeschwindigkeit. Beschleunigen ist aber nur
     * möglich, wenn die Laderampe geschlossen ist; sonst passiert gar nichts.*/
   override def beschleunige (deltaV: Int) =
      if (laderampeOffen == false) then
         vMom += deltaV
         vMom min vMax

   /** Belädt das Fahrzeug mit einem Stück Fracht.
   * Es kann höchstens ein Frachtstück geladen werden. 
   * Der LKW lädt nur etwas ein, falls er noch nicht beladen ist UND falls die
   * Laderampe offen ist.
   * Das eingeladene Frachtstück wird beim Einladen in der Greenfoot-World
   * unsichtbar. 
   * 
   * @param frachtstueck Zeiger auf das Ladegut, das aufgenommen werden soll.
   * @return ob der Beladevorgang erfolgreich war, d.h. ob 
   * das angegebene Frachtstueck eingeladen werden konnte.     */
   def beladeMit (frachtstueck: Ladegut_S): Boolean =
      ???

   /** Entlädt das Fahrzeug.
    * 
    * Die Funktion lädt das Ladegut aus dem Fahrzeug aus, falls eines drin ist.
    * Das Fahrzeug ist danach unbeladen.
    * 
    * Anschließend setzt die Methode das ausgeladene Ladegut neben der
    * Position dieses LKW in die Welt und macht es damit wieder sichtbar.
    *  
    * @return das Frachtstueck, das dieses Fahrzeug vorher 
    * geladen hatte; null, wenn dieses Fahrzeug leer war.     */
   def entlade(): Ladegut_S =
      ??? // TODO


class PKWS extends KfzS(50, 100, 0.1)
29.05.25, 21:22 - sammy: Hey, du warst doch so mit ego akim stefan, luka in ner gruppe wegen florenz, wie macht ihr es wegen zimmern?
29.05.25, 23:04 - Ole: also ich bin mit max und luka in nem zimmer
22.06.25, 18:35 - Ole: kannst du mir bitte das blatt von info schicken?
22.06.25, 18:36 - sammy: ‎IMG-20250622-WA0010.jpg (Datei angehängt)
22.06.25, 18:36 - sammy: Hier
22.06.25, 18:37 - Ole: dankeschön
22.06.25, 18:37 - sammy: Bin mir aber nicht mehr sicher bis wohin wir machen sollten
22.06.25, 18:37 - sammy: Ich glaube 8
22.06.25, 18:38 - Ole: ok, alles klar
01.07.25, 11:45 - Ole: ‎IMG-20250701-WA0002.jpg (Datei angehängt)
was hast du da?
01.07.25, 11:46 - sammy: wenns val ist, ist es unveränderlich, dann ist private nicht so wichtig da man nur auslesen kann
01.07.25, 11:47 - Ole: achso, danke
06.07.25, 12:31 - Ole: hast du für inf schon die methode gesundheitanpassen gemacht? also a2.2?
06.07.25, 12:42 - Ole: und die aufgaben danach verstehe ich ehrlich gesagt auch nicht. bist du schon fertig mit den aufgabne?
06.07.25, 12:43 - sammy: ich bin grad an 2.2 lol
06.07.25, 12:43 - sammy: du hast ein timing
06.07.25, 12:44 - sammy: ```
def gesundheitanpassen(a: Int): Unit = 
  var Gesundheit = 100
  var Leben = 3
  
  Gesundheit = Gesundheit + a
  if Gesundheit > 100 then
    Gesundheit = 100
    return
  if Gesundheit > 1 then return
  if Leben < 1 then
    showMessage("Game Over")
    return
  
  Leben -= 1
  Gesundheit = 100
  return
```
06.07.25, 12:44 - sammy: ich hätte gedacht so
06.07.25, 12:47 - Ole: ja das sieht gut aus, dankeschön
06.07.25, 21:44 - Ole: hast du a3 gemacht? ich check das nicht
06.07.25, 22:50 - sammy: Ja
06.07.25, 22:50 - sammy: Schick mal Aufgabe
06.07.25, 22:50 - Ole: ‎IMG-20250706-WA0043.jpg (Datei angehängt)
06.07.25, 22:51 - sammy: Ich hab das schon weggeräumt
06.07.25, 22:51 - sammy: Dbake
06.07.25, 22:51 - sammy: Das mit dem array?
06.07.25, 22:51 - Ole: ja
06.07.25, 22:51 - Ole: aber das mit der methode kann ich auch nicht
06.07.25, 22:51 - sammy: Array.ofDim[String](10, 20)
06.07.25, 22:52 - Ole: ist das alles was man braucht für die erste aufgabe?
06.07.25, 22:52 - sammy: Das ist eig nicht so schwer, du braucht eine Funktion die das array durchläuft und überprüft das nur zeichen drin sind die drin sein dürfen
06.07.25, 22:52 - sammy: Ja
06.07.25, 22:52 - Ole: ok, dankeschön
06.07.25, 22:52 - sammy: 10 * 20 spielfeld ergibt das
06.07.25, 22:53 - sammy: Jo gerne
06.07.25, 22:53 - sammy: Also das is die theorie, ich hatte aber kb das zu machen aber so würde es gehen
06.07.25, 22:55 - Ole: achso 👍
08.07.25, 12:02 - Ole: ‎IMG-20250708-WA0001.jpg (Datei angehängt)
wie würdest du das machen? hab es selbst probiert aber es kommen nur fehlermeldungen
08.07.25, 12:11 - Ole: ‎IMG-20250708-WA0002.jpg (Datei angehängt)
und weißt du warum das nicht geht?
08.07.25, 12:12 - Ole: ist nicht so dringend, du kannst es dir angucken wenn du selber die aufgaben machst
08.07.25, 12:17 - sammy: Was steht beim fehler?
08.07.25, 12:18 - Ole: hab das schon gelöscht, aber es kamen irgendwie 12 fehler die im code von den anderen klassen den Punkt, wenn ich ihn verwendet habe, als fehler gemeldet haben
08.07.25, 12:18 - Ole: also die klasse punkt
08.07.25, 12:19 - sammy: Okay
08.07.25, 12:19 - sammy: Warte
08.07.25, 12:21 - sammy: ```m
// Geometrie als OOP-Fallstudie

// ================================================

// Geom Object

// ================================================

// Aufgabe 1
// Definieren Sie eine immutable (das heißt: die Instanzen sollen nach der
// Konstruktion nicht veränderlich sein) Klasse Punkt mit zwei Double-
// Koordinaten x und y. Überschreiben Sie die toString-Methode so, dass
// ein Punkt in der Form (3.1 | 5) als String dargestellt wird.
// Ergänzen Sie eine Methode abstandZu(p: Punkt), die den Abstand von
// this zu p berechnet und zurückgibt.

class Punkt(val x: Double, val y: Double):
  override def toString: String =
    return "(" + x.toString + " | " + y.toString + ")"

  def abstandZu(p: Punkt): Double =
    val diffx = p.x - this.x
    val diffy = p.y - this.y

    var tmp = (diffx * diffx + diffy * diffy)
    return Math.sqrt(tmp)

// Punkt
val p1 = Punkt(7,8)
val p2 = Punkt(3.1,5)

print(p1.abstandZu(p2).toString)

// ================================================

// Begründen Sie, dass bei einer mutablen Klasse die Entscheidung zwischen
// private- und public-Attribute wichtiger ist als bei einer immutablen Klasse.

// Ja denn wenn die immutable ist, können deren Variablen nicht verändert
// werden

// ================================================

// Definieren Sie eine immutable Klasse Gerade, die durch zwei Punkte
// definiert wird. Überschreiben Sie die toString-Methode so, dass eine
// Gerade in der Form "Gerade durch (3.1 | 5) und (2 | 2)" als String
// dargestellt wird.

class Gerade(val p1: Punkt, val p2: Punkt):
  override def toString: String =
    return ("Gerade durch " + p1.toString + " und " + p1.toString)

// Gerade
val g1 = Gerade(p1, p2)
print(g1.toString)

// ================================================

// Definieren Sie eine immutable Klasse Strecke, die durch zwei Punkte
// definiert wird. Überschreiben Sie die toString-Methode so, dass die
// Instanz in der Form Strecke von (3.1 | 5) nach (2 | 2) als String
// dargestellt wird. Ergänzen Sie eine Methode länge. Ergänzen Sie eine
// Methode mittelsenkrechte. Welchen Rückgabetyp muss diese Methode haben?

class Strecke(f1: Punkt, f2: Punkt) extends Gerade(f1, f2):
  override def toString: String =
    return ("Strecke von " + f1.toString + " und " + f2.toString)

  def laenge(): Double =
    val diffx = f1.x - f2.x
    val diffy = f1.y - f2.y

    var tmp = (diffx * diffx + diffy * diffy)
    var tp2 = Math.sqrt(tmp)
    if tp2 < 0 then return (tp2 * (-1))
    return tp2

  def mittelsenkrechte() = ???

// Strecke
// testing

// ================================================

// Definieren Sie eine immutable Klasse Kreis mit Mittelpunkt und Radius.
class Kreis(val mittelpunkt: Punkt, val radius: Double):

  // Nebenkostruktor, der einen Kreis aus mittelpunkt und umfang erzeugt:
  def this(u: Double, m: Punkt) /* kein Typ!! */ =
    // Nebenkonstruktor muss Hauptkonstruktor aufrufen
    this(m, u / 2 / 3.14)

  // Nebenkonstruktor aus mittelpukt und punkt im umfang
  def this(m: Punkt, up: Punkt) =
    this(m, up.abstandZu(m))

// Kreis
// testing

// ================================================

// Neuer Stoff: Nebenkonstruktoren ("secondary constructors")
// und Factory-Methoden.
// Die Klasse Kreis hat genau einen Konstruktor, eben den Klassenkopf.
// Implementieren Sie zwei weitere Möglichkeiten, Kreis-Instanzen zu
// konstruieren: Einmal mit einem "sekundären" Konstruktor, einmal mit einer
// Klassenmethode im Companion-Objekt der Klasse Kreis.

// Neuer Stoff: Vorgabeparameter, copy-Methode für immutable Klassen
// Instanzen immutabler Klassen kann man nicht verändern. Es ist aber hilreich,
// veränderte neue Instanzen erzeugen zu können. Dazu dient die copy-Methode.
// Sie hat Parameter
```
08.07.25, 12:21 - sammy: das hab ich
08.07.25, 12:23 - Ole: wo ist die klasse? ich finds nicht
08.07.25, 12:25 - sammy: welche?
08.07.25, 12:25 - Ole: also die oberklasse geomobjekt
08.07.25, 12:26 - sammy: hab ich noch nicht
08.07.25, 12:26 - Ole: achso
08.07.25, 12:26 - Ole: aber danke trotzdem für den code
08.07.25, 12:42 - sammy: bist du eig weiter?
08.07.25, 12:42 - sammy: ```
// Geom Object
abstract class GeomObject:
  var farbe: String = "rot"
  var dicke: Int = 1
```
08.07.25, 12:42 - sammy: ich hab so geacht
08.07.25, 12:43 - Ole: mit der oberklasse nicht, aber ich mach grad die anderen aufgaben
08.07.25, 12:43 - Ole: ich probiers gleich mal aus, dankeschön
08.07.25, 12:44 - sammy: ah ok
14.07.25, 10:24 - Ole: ‎IMG-20250714-WA0001.jpg (Datei angehängt)
sry hätte nochmal ne frage zu inf. ich hab isEmpty jetzt mal so gemacht, aber ich weiß nicht wie ich das testen soll. kannst du mir da vll helfen?
14.07.25, 10:33 - Ole: ‎IMG-20250714-WA0002.jpg (Datei angehängt)
und kannst du, wenn du fertig bist mir mal deins schicken, damit ichs vergleichen kann
14.07.25, 11:40 - sammy: zum überfrüfen musst du eine liste erstellen und dann die funktion in der liste ausführern
14.07.25, 11:48 - sammy: Du hast diese Nachricht gelöscht.
14.07.25, 11:48 - sammy: Du hast diese Nachricht gelöscht.
14.07.25, 11:49 - sammy: wait
14.07.25, 11:52 - sammy: ``````
14.07.25, 11:52 - sammy: ```
// 2025.07.14

class Listenknoten[T](var inhalt: T):
  var next: Listenknoten[T] = null

class Liste[T]():
  private var Kopf: Listenknoten[T] = null

  def isEmpty(): Boolean =
    if Kopf == null then return true
    false
  
  def insert(element: T): Unit =
    val tmp = Kopf
    Kopf = Listenknoten[T](element)
    Kopf.next = tmp
  
  def contains = ???
  
  def clear = ???
  
//=============================================
//
// Testing  
//
//=============================================

var l1 = Liste[Int]

l1.isEmpty()

l1.insert(2)

l1.isEmpty()
```
14.07.25, 11:52 - sammy: so hab ich gemacht
14.07.25, 13:15 - Ole: alles klar, dankeschön
20.07.25, 13:17 - Ole: kannst du mir bitte deinen code von append bis insertat schicken?
20.07.25, 13:17 - Ole: das sollten wir doch als hausaufgabe machen oder?
20.07.25, 13:18 - sammy: was wir genau machen sollten bin ich mir grad nich sicher
20.07.25, 13:19 - Ole: also ich glaube wir sollten bis zu insertat machen
20.07.25, 13:23 - Ole: kannst du mir bitte schicken, wie du das bei get gemacht hast?
20.07.25, 13:23 - sammy: get hab ich nich
20.07.25, 13:23 - sammy: also noch nich
20.07.25, 13:24 - Ole: achso
20.07.25, 13:24 - Ole: trotzdem danke
21.07.25, 10:19 - Ole: verstehst du das mit dem UML Diagramm StackMitListe?
21.07.25, 10:33 - sammy: ja
21.07.25, 10:34 - sammy: wir sollen so ein diagramm wie wir auch zu liste und listenknoten gemacht haben
21.07.25, 10:34 - sammy: über die klasse eben
21.07.25, 10:35 - Ole: wie würde das dann aussehne? wir wissen doch nichts außer die methoden oder?
21.07.25, 10:35 - sammy: stacklist is basicly dasselbe
21.07.25, 10:35 - sammy: Du hast diese Nachricht gelöscht.
21.07.25, 10:36 - sammy: ich hab die schon runtergeschrieben in kunst weil das so langweillig is
21.07.25, 10:36 - sammy: aber nich kopieren, damit lernst du nix
21.07.25, 10:36 - Ole: achso
21.07.25, 10:36 - sammy: nur zum anschauen
21.07.25, 10:36 - Ole: klar, mach ich nicht
21.07.25, 10:36 - Ole: aber sollten wir das schon machen oder nur das diagramm?
21.07.25, 10:36 - sammy: ne, nur diagramm so weit ich weiß
21.07.25, 10:37 - Ole: ok, dankeschön
21.07.25, 10:47 - Ole: ‎IMG-20250721-WA0003.jpg (Datei angehängt)
hab das jetzt so gemacht, bin mir nur bei public und private nicht sicher
21.07.25, 10:55 - sammy: ich glaub auch so
27.07.25, 12:10 - Ole: hey, hast du noch den zettel von informatik vom montag, wo wir diesen kleinen test gemacht haben mit den begriffen und so? würde mir das gerne nochmal anschauen, aber habe meins nicht vollständig
27.07.25, 12:38 - sammy: Also die fragen hab ich auch nich mehr
27.07.25, 12:39 - sammy: ‎IMG-20250727-WA0004.jpg (Datei angehängt)
27.07.25, 12:45 - Ole: dankeschön
14.09.25, 13:40 - sammy: Hey, ich hab gesehen du bist auch in dem mathe kurs in der ersten woche
14.09.25, 13:40 - sammy: schwänzt du inf am mittwoch?
14.09.25, 17:17 - Ole: Wann ist der mathe kurs nochmal?
14.09.25, 17:17 - Ole: Nur mitwochs?
14.09.25, 17:18 - sammy: Ne dienstag, Mittwoch, Donnerstag
14.09.25, 17:22 - Ole: Weiss noch nicht, wahrscheiblich geh ich inf
16.09.25, 19:42 - Ole: Kannst du mir bitte das von info von vor den ferien schicken?
16.09.25, 19:43 - sammy: Ah ja stimmt
16.09.25, 19:43 - sammy: Maxh ich
16.09.25, 19:46 - sammy: Bis wann willst du so haben?
16.09.25, 19:46 - sammy: Juni auch noch?
16.09.25, 19:46 - sammy: 
16.09.25, 19:46 - sammy: ‎IMG-20250916-WA0033.jpg (Datei angehängt)
16.09.25, 19:46 - sammy: ‎IMG-20250916-WA0034.jpg (Datei angehängt)
16.09.25, 19:46 - sammy: ‎IMG-20250916-WA0035.jpg (Datei angehängt)
16.09.25, 19:46 - sammy: ‎IMG-20250916-WA0036.jpg (Datei angehängt)
16.09.25, 19:46 - sammy: ‎IMG-20250916-WA0037.jpg (Datei angehängt)
16.09.25, 19:46 - sammy: ‎IMG-20250916-WA0038.jpg (Datei angehängt)
16.09.25, 19:46 - sammy: ‎IMG-20250916-WA0039.jpg (Datei angehängt)
16.09.25, 19:47 - sammy: Ivh schau jzt noch aufm Laptop
16.09.25, 19:48 - sammy: willst du das aus der vbox auch?
16.09.25, 19:50 - Ole: Wenn das für dich keine umstände macht gern
16.09.25, 19:50 - Ole: Ne danke nur die letzten 2 drei stunden vor den ferien
16.09.25, 19:50 - Ole: Vielen dank samuel
16.09.25, 19:52 - sammy: Joa gerne
16.09.25, 19:52 - sammy: Ah ok
16.09.25, 19:52 - sammy: I mean ich kann dir morgen erste auch den Ordner geben und du gibst mir den dann um 2 wieder
16.09.25, 20:01 - Ole: ne danke, die blätter die du mir grade geschickt hast reichen mir schon
16.09.25, 20:06 - sammy: Passt
17.09.25, 14:29 - sammy: Du hast diese Nachricht gelöscht.
17.09.25, 14:39 - sammy: https://1drv.ms/u/c/dae2fdd7e6b8aac0/EcFFeuj3VZVHmj6i1Na8APMBKng6wF-dHAB-dUMbLcfKEw?e=rWHf3N
28.09.25, 17:58 - Ole: hey samuel, ich sitz grad an der inf hausaufgabe. wie soll man das mit den bäumen dann testen? haben wir schonmal so bäume erstellt im code? oder wie hast du das gemacht?
28.09.25, 18:00 - sammy: Du musste einen baum knoten erstellen und dan hängst du dann an den nächsten
28.09.25, 18:00 - sammy: Ich schick gleich ein bild
28.09.25, 18:02 - sammy: ‎IMG-20250928-WA0006.jpg (Datei angehängt)
so zb
28.09.25, 18:02 - Ole: achso, dankeschön
28.09.25, 18:03 - sammy: der baum sähe dann so aus
28.09.25, 18:03 - sammy: ‎IMG-20250928-WA0007.jpg (Datei angehängt)
28.09.25, 18:03 - sammy: gerne, kein problem
28.09.25, 19:04 - Ole: wie hast du queue gemacht? das war doch auch hausaufgabe oder?
29.09.25, 07:17 - sammy: Shit
29.09.25, 07:18 - sammy: Ich hab das vergessen
29.09.25, 07:18 - sammy: Also ich habs gar nich
29.09.25, 07:18 - sammy: Sorry
29.09.25, 07:37 - Ole: Alles gut
29.09.25, 07:38 - Ole: Ich komm heute eh nicht, bin krank. Aber kannst du mir es bitte schicken falls du es noch machst?
29.09.25, 07:39 - sammy: ja ich schicks dir
29.09.25, 08:04 - Ole: Danke
29.09.25, 10:14 - sammy: ```
// BTree 29.09.2025

// "Begleitobjekt" der Klasse BTree:
// alle Methoden hier können auch ohne BTree-Instanz aufgerufen werden:
object BTree:
  def groesse[T](akt: BTree[T]): Int = {
    if akt == null then return 0
    else 1 + groesse(akt.links) + groesse(akt.rechts)
  }

  def contains[T](wert: T, akt: BTree[T]): Boolean = {
    if akt == null then return false
    if akt.inhalt == wert then return true
    contains(wert, akt.links) || contains(wert, akt.rechts)
  }
  
  def isEmpty[T](akt: BTree[T]): Boolean = groesse(akt) == 0

// zweiter und dritter Parameter mit Default Wert hinter dem "="
class BTree[T](var inhalt: T, var links: BTree[T] = null, var rechts: BTree[T] = null):
  // Methode groesse ist "überladen"
  def groesse: Int =
    BTree.groesse(this)
    
  def contains(wert: T): Boolean =
    BTree.contains(wert, this)
    
  // Tiefe des Knotens  
  // def depths: Int =

// Methode ist jzt im Companion Object
/*
private def groesse(akt: BTree[T]): Int = {
  var tmp = 0

  if akt.links == null then
    tmp += 0
  else
    tmp += groesse(akt.links)

  if akt.rechts == null then
    tmp += 0
  else
    tmp += groesse(akt.rechts)

  return (tmp + 1)
}
*/
  

  // Methode ist jzt im Companion Object
  /*
  private def contains(wert: T, akt: BTree[T]): Boolean = {
    if akt.inhalt == wert then return true
    
    else if contains(wert, akt.links) == true then return true
    else if contains(wert, akt.rechts) == true then return true
      
    else return false
  }
  */  

// Tests
val b4 = BTree[Int](5)  
val b2 = BTree[Int](2, b4, b4)  
val b1 = BTree[Int](1, b2, b2)
b1.groesse
b1.contains(2)
b1.contains(5)

val b0: BTree[Int] = null
BTree.groesse(b0)
BTree.contains(9, b0)
BTree.isEmpty(b0)

// Kind nachträglich anhängen
b4.rechts = b4
b1.groesse

```
29.09.25, 10:15 - sammy: das is BTree was wir gemacht haben
29.09.25, 10:15 - sammy: er hat noch ein blatt ausgeteilt, das schick ich nacher
29.09.25, 10:22 - sammy: ah und als ha
29.09.25, 10:22 - sammy: size, contains, isEmpty, depths
29.09.25, 10:22 - sammy: ich hab die schon in meinem, also kopier die nicht einfach bitte, also wäre besser für dich
29.09.25, 10:23 - sammy: und wir sollen halt alles noch testen wie immer halt
29.09.25, 10:23 - sammy: aber über arbeit hat er nix gesagt
29.09.25, 10:28 - Ole: Alles klar, vielen dank
29.09.25, 10:28 - Ole: Ne mach ich nicht, ich mach es selber
02.10.25, 19:38 - sammy: Hey kannst du mir bitte schicken was du für inf arbeit aufgeschrieben hast?
03.10.25, 12:24 - Ole: stimmt, tut mir leid habs vergessen
03.10.25, 12:24 - Ole: - alles aus dem Thema "Datenstrukturen"
- auch Implementierungen von z.B. Methoden
- Traversierung und so was angeben können
- einfache Aufgaben zu dem Aufwand (Bsp: Welcher Aufwand erzeugt Suchen? Bestcase / Worstcase)
- Frage zu binären Suchbäumen kommt wahrscheinlich dran
- Verständnisfragen zu allen Datenstrukturen (Bsp: Wieso gibts zu Binärbäumen ne      Inordertraversierung aber bei beliebigen Bäumen nicht?)
- nichts von vor den Datenstrukturen
- UML lesen können, aber mehr nicht zu OOP
- object verstehen
- preorder implementieren wahrscheinlich
- Übungsaufgaben mit Musterlösung auf Moodle
- "Inselrundfahrt" selbst testen
03.10.25, 12:25 - Ole: ich weiß auch nicht ob das alles ist, aber mehr hab ich nicht mitgeschrieben
03.10.25, 13:14 - sammy: Danke
03.10.25, 13:14 - sammy: Kein problem
04.10.25, 19:04 - Ole: ‎IMG-20251004-WA0008.jpg (Datei angehängt)
Hey samuel, mein laptop ging heute morgen einfach random nicht. Ich hab mehrmals neugestartet aber hat nichts gebracht. Jetzt geht es immer noch nicht, weißt du was ich hier drücken soll? Ich hab angst dass ich irgendwie datein verlieren könnte
04.10.25, 19:05 - sammy: Fortsetze
04.10.25, 19:05 - sammy: Dann geht es normalerweise zum Betriebssystem weiter sofern es noch funktioniert
04.10.25, 19:06 - Ole: Ok danke
04.10.25, 19:21 - Ole: Ich bin jetzt angemeldet aber dann ist mein bild eingefroren. Das war heut morgen auch schon. Ich kann nur noch die maus bewegen und wenn ich auf die taskleiste gehe kommt das ladesymbol. Weißt du vll was man da macht?
04.10.25, 19:23 - sammy: What
04.10.25, 19:23 - sammy: Also hmm mir fällt da auf die schnelle nichts ein
04.10.25, 19:23 - sammy: Äh kannst du maybe mal ein bild machen oder kurzes video machen
04.10.25, 19:24 - Ole: ‎VID-20251004-WA0009.mp4 (Datei angehängt)
Das ist jetzt die ganze zeit stuck so
04.10.25, 19:25 - sammy: Versuche nochmal shift drücken und dann geh auf Neustart dann solltest du wieder in den bluescreen kommen und such nach automatische Reperatur, hilft vielleicht aber wahrscheinlich nicht
04.10.25, 19:26 - sammy: Bekommst du task manager auf mit strg alt f?
04.10.25, 19:26 - Ole: Ne ich kann nichts drücken
04.10.25, 19:26 - Ole: Ich probiers mal, danke schonmal
04.10.25, 20:26 - sammy: Hat es bei dir eig funktioniert?
04.10.25, 20:27 - sammy: Windows is manchmal ziemlich und ohne admin rechte kann man da halt eig nix machen außer neustarten und hoffen das funktioniert
04.10.25, 20:27 - sammy: Aber sag falls was neues anderes kommt
04.10.25, 20:41 - Ole: Ne hat nicht funktioniert, ich probier es morgen nochmal
04.10.25, 20:41 - Ole: Mach ich, danke für deine hilfeA
05.10.25, 15:25 - Ole: jetzt gehts aufeinmal wieder, keine ahnung was los war
05.10.25, 16:00 - sammy: ich auch nicht, das liegt irgendwo an Windows und der schulsoftware, aber genauer weiß ich es auch nicht, aber nice das es wieder geht
06.10.25, 20:48 - Ole: ich habe meinen pc jetzt mal angelassen und nach ca. 10 minuten ging es dann plötzlich. das problem ist jetzt, dass es jedes mal 10 minuten braucht um anzumelden. ich habe mal gegooglet und es hieß dass man das schnellstarten von windows deaktivieren soll. das geht aber nur mit adminrechten. weißt du was man sonst versuchen könnte? ansonsten gehe ich wahrscheinlich morgen zu herr fesenbeck
06.10.25, 20:48 - Ole: ‎IMG-20251006-WA0029.jpg (Datei angehängt)
oder weißt du ob ich hier was bei autostart deaktivieren kann, was nicht notwendig ist?
06.10.25, 20:50 - sammy: ne, die restlichen 5 wären glaub nich so gut ausszuamchen, die beiden .vbs, Dateien kann man eig ausmachen, also die machen nur schrott (unwichtiges zeug mein ich) aber das geht nich ohne admin rechte
06.10.25, 20:51 - Ole: achso, danke
06.10.25, 20:51 - sammy: öffne mal den explorer
06.10.25, 20:51 - sammy: paste das in die leiste
06.10.25, 20:51 - sammy: C:\paedML_UOFF
06.10.25, 20:52 - sammy: gehe oben auf eigenschafften und schau wie groß der is
06.10.25, 20:52 - sammy: dann lösch den Content davon, das is probably das gleiche wie bei akin, ich schätze das ist das backup
06.10.25, 20:53 - Ole: 6,1 GB
06.10.25, 20:53 - sammy: lösch
06.10.25, 20:53 - sammy: is unwichtig
06.10.25, 20:53 - Ole: was davon? alle drei ordner?
06.10.25, 20:53 - sammy: alle 4
06.10.25, 20:54 - sammy: ne wait
06.10.25, 20:54 - sammy: stop
06.10.25, 20:54 - sammy: ne doch alle 4
06.10.25, 20:54 - Ole: bei mir sind es nur 3
06.10.25, 20:54 - sammy: bzw 3 wenns dir nur 3 anzeigt weil einer versteckt is
06.10.25, 20:54 - sammy: geh oben auf ansicht
06.10.25, 20:54 - sammy: ausgeblendete ellemente
06.10.25, 20:54 - Ole: stimmt sind 4
06.10.25, 20:55 - Ole: sollte ich davor ein backup von irgednwas machen?
06.10.25, 20:55 - sammy: von den ordner nicht, das ist eine Kopie von %USERPROFILE%
06.10.25, 20:55 - Ole: achso
06.10.25, 20:56 - sammy: ‎IMG-20251006-WA0030.jpg (Datei angehängt)
06.10.25, 20:56 - sammy: du könntest das verändern
06.10.25, 20:57 - Ole: da war ich schon, aber für den schnellstart deaktivieren muss ich admin rechte haben
06.10.25, 20:57 - sammy: du kannst auch ruhemodus anschalten, geht zwar nur mit Password aber auch da gibt's möglichkeiten
06.10.25, 20:57 - sammy: ja es gibt so möglichkeiten
06.10.25, 20:58 - Ole: habs gelöscht, ich starte jetzt mal neu, danke schonmal
06.10.25, 20:59 - sammy: ah ja dann schau obs wieder da ist
06.10.25, 20:59 - Ole: Ok mach ich
06.10.25, 21:10 - Ole: geht leider nicht, hat immer noch so lange gedauert. von den ordnern ist nur noch der userprofile ordner da aber das ist der mit den 6 GB.
06.10.25, 21:29 - sammy: oh
06.10.25, 21:30 - sammy: dann wars falsch
06.10.25, 21:30 - sammy: geh mal bis hier rein und lösch weg
06.10.25, 21:30 - sammy: C:\paedML_UOFF\UserProfile\UOFF.V6
06.10.25, 21:37 - sammy: oder das
06.10.25, 21:37 - sammy: C:\paedML_UOFF\UserProfile
06.10.25, 21:37 - sammy: ich bin mir nicht sicher wo man genau weglöschen muss das es nicht mehr neu erstellt leider
06.10.25, 21:37 - Ole: ok alles gut
06.10.25, 21:37 - Ole: das sind ja alles nur kopien oder?
06.10.25, 21:38 - sammy: ja
06.10.25, 21:38 - sammy: von %USERPROFILE%
06.10.25, 21:39 - Ole: ich probiers später mal, möchte jetzt noch nicht neustarten, weil ich grad am lernen bin. aber vielen dank, wirklich!
06.10.25, 21:40 - sammy: Jo no prob, ich würde generell nich so oft ausmachen, glaub meiner hat durschnittlich so ne runtime von ner woche
06.10.25, 21:41 - Ole: ok 👍
07.10.25, 10:07 - sammy: ich hab mal geschaut 😂
07.10.25, 10:07 - sammy: 10 tage 20 stundne
07.10.25, 11:37 - Ole: Das ist krass 😅
07.10.25, 11:41 - sammy: joa, i mean es geht und meine Programme sind dann noch offen
07.10.25, 11:47 - Ole: Stimmt
07.10.25, 11:47 - Ole: Aber man sollte den doch nicht in der pc tasche anlassen wegen dem überhitzen
07.10.25, 11:49 - sammy: joa aber der wird nicht warm wegen energiesparmodus wenn man den zuklappt
07.10.25, 11:51 - Ole: Achso
08.10.25, 08:31 - sammy: funktioniert dein Laptop eigentlich wieder?
08.10.25, 09:29 - Ole: Dauert immer noch so lange
08.10.25, 09:29 - Ole: Aber ich lass ihn jetzt einfach an und restarte ihn nur so alle 2 3 tage. Dann geht das
08.10.25, 16:21 - sammy: ah joa
08.10.25, 16:21 - sammy: passt
08.10.25, 19:32 - Ole: ‎IMG-20251008-WA0003.jpg (Datei angehängt)
08.10.25, 19:32 - Ole: Das kam grad plötzlich
08.10.25, 19:35 - Ole: ‎IMG-20251008-WA0004.jpg (Datei angehängt)
Soll ich das aktivieren?
08.10.25, 19:35 - sammy: Ne lass
08.10.25, 19:35 - sammy: Ich weiß tbh nich was das macht und wir sollen das eig eh lassen <Diese Nachricht wurde bearbeitet.>
08.10.25, 19:36 - sammy: Hast du was runtergeladen?
08.10.25, 19:36 - Ole: Ne
08.10.25, 19:36 - Ole: Ich hab opera gx grad deinstalliert weil ich speicherplatz brauchte und dann hat es kurz gelaggt und die nachricht kam
08.10.25, 19:37 - Ole: Da stand ja auch irgendwas von opera
08.10.25, 19:37 - sammy: Joa dann liegt es wahrscheinlich daran, opera würde ich aber auch zutrauen das es ne spyware is
08.10.25, 19:37 - sammy: Aus china und schlimmer als google tbh
08.10.25, 19:38 - Ole: Ok was soll ich jetzt machen?
08.10.25, 19:38 - Ole: Es ignorieren?
08.10.25, 19:38 - sammy: Ja, wenn du willst kannst du aber auch gdata gehen und dann Virenprüfung wenn du sichergehen willst
08.10.25, 19:39 - sammy: Das würd ich machen
08.10.25, 19:39 - Ole: Ok mach ich, danke
08.10.25, 19:39 - Ole: Irgendwie geht grad nichts
08.10.25, 19:39 - Ole: ‎IMG-20251008-WA0005.jpg (Datei angehängt)
08.10.25, 19:54 - Ole: ‎IMG-20251008-WA0006.jpg (Datei angehängt)
Das sieht gut aus
08.10.25, 19:59 - sammy: Jup
08.10.25, 20:00 - Ole: Sollte ich trotzdem zu herr fesenbeck?
08.10.25, 20:01 - sammy: Nur deswegen? Nein, würde ich nicht
08.10.25, 20:03 - sammy: wenn du sichergehen willst
08.10.25, 20:03 - sammy: geh da auf virenprüfung
08.10.25, 20:03 - sammy: und dann rechner prüfen
08.10.25, 20:03 - sammy: ‎IMG-20251008-WA0007.jpg (Datei angehängt)
08.10.25, 20:05 - Ole: Ok mach ich
08.10.25, 20:05 - Ole: Vielen dank
08.10.25, 20:06 - sammy: kein problem
08.10.25, 20:08 - Ole: Ich hab übrigens versehentlich den famila ordner für meine virtual box verschoben und kann sie jetzt nicht  mehr öffnen. Weiß du was man da macht? Ich kann den dateipfad irgendwie nicht umlinken
08.10.25, 20:11 - sammy: du hast die vbox Dateien noch aber die softwaer hat noch den alten pfad?
08.10.25, 20:11 - sammy: ‎IMG-20251008-WA0008.jpg (Datei angehängt)
du entfernst die erst
08.10.25, 20:12 - sammy: und dann gehst du oben auf hinzufügen wählst den neuen pfad aus
08.10.25, 20:12 - Ole: Ok danke
08.10.25, 20:12 - Ole: Muss ich dann die passwörter neu eingeben?
08.10.25, 20:12 - sammy: für Login? nein
08.10.25, 20:13 - Ole: Achso alles klar
08.10.25, 20:13 - sammy: ‎STK-20250807-WA0045.webp (Datei angehängt)
08.10.25, 20:14 - sammy: hast du eig schon inf angefangen?
08.10.25, 20:14 - Ole: Ne noch nicht
08.10.25, 20:14 - sammy: ich auch nich
08.10.25, 20:15 - Ole: Will heute lieber noch bisdchen mathe lernen wahrscheunlich fang ich morgen an
08.10.25, 20:15 - sammy: ah ja ihr schreibt ja n test
08.10.25, 20:15 - sammy: Viel glück
08.10.25, 21:42 - Ole: dankeschön
11.10.25, 22:04 - Ole: hey samuel, ich bin grad am inf lernen. ich hab die toString methode übernommen um zu testen, aber trotzdem bekomme ich das hier val res2: String = Liste@739adf10 wenn ich liste1.toString mache. weißt du woran das liegt?
11.10.25, 22:05 - sammy: Bin grad am Geburtstag feiern, ich antworte morgen
11.10.25, 22:08 - Ole: alles klar
11.10.25, 22:08 - Ole: hast du geburtstag?
11.10.25, 22:46 - sammy: Ja
11.10.25, 22:55 - Ole: Dann alles Gute 🥳🥳
11.10.25, 23:01 - sammy: Danke
12.10.25, 01:54 - Ole: ist egal habs jetzt selbst bemerkt woran es lag
12.10.25, 01:54 - Ole: glaubst du man muss die toString methode können?
12.10.25, 02:02 - Ole: und warum soll die methode remove bei liste ein boolean sein?
12.10.25, 03:37 - Ole: und könntest du mir bitte falls du kurz zeit hast erklären was StackListBase ist und was die funktion dieser extra klasse ist?
12.10.25, 13:04 - sammy: Ich schätze eher nicht
12.10.25, 13:04 - sammy: Schick mal ein Bild
12.10.25, 13:05 - sammy: Wie kommst du darauf?
12.10.25, 13:53 - Ole: ‎IMG-20251012-WA0095.jpg (Datei angehängt)
das ist stacklistbase
12.10.25, 13:53 - Ole: ‎IMG-20251012-WA0096.jpg (Datei angehängt)
und das die remove methode
12.10.25, 13:53 - Ole: hast du viel gelernt?
12.10.25, 14:00 - sammy: ich hatte vor nacher anzufangen
12.10.25, 14:11 - sammy: das is eif ein listenknoten
12.10.25, 14:11 - sammy: den ich andres benannt habe
12.10.25, 14:12 - sammy: das sollte eig nicht so sein, zw ich mir nicht sicher
12.10.25, 14:12 - sammy: wahrscheinlich wäre da unit schlauer
12.10.25, 15:31 - Ole: Achso alles klar, dankeschön
12.10.25, 17:00 - Ole: du hast bei stackmitArray ja so befehle benutzt wie ArrayBuffer, Array.ofDim oder as.Instance of. muss man das können oder gibt es auch andere möglichkeiten das zu schreiben?
12.10.25, 17:00 - Ole: sorry ich frag echt viel, wenn du mal fragen hast, auch in einem anderen kurs kannst du mir gerne schreiben
12.10.25, 17:01 - sammy: Das mit arraybuffer war helen und den sollen wir nich benutzen
12.10.25, 17:01 - Ole: achso, und die anderen sachen?
12.10.25, 17:05 - sammy: as.instance of ist ein upcast weil Scala sozusagen über Java läuft und in Java gibt es keine dynamische typisierung
12.10.25, 17:07 - Ole: also braucht man das?
12.10.25, 17:08 - sammy: ja
12.10.25, 17:08 - Ole: ok danke
12.10.25, 17:09 - sammy: Maybe frag ihn mal, weil er hat selbst gesagt dass das ziemlich dumm is und ich kann das auch nich gut erklären
12.10.25, 17:09 - sammy: aber es ist halt die einzige Möglichkeit das zu machen
12.10.25, 17:10 - Ole: achso, ne dann lern ich das einfach
12.10.25, 17:11 - sammy: ‎PTT-20251012-WA0135.opus (Datei angehängt)
12.10.25, 17:14 - Ole: Ah okay, vielen dank
12.10.25, 23:07 - sammy: ```
class StackArray[T](capacity: Int):
  private val arr: Array[Any] = new Array[Any](capacity)
  private var topIndex: Int = -1

  def push(elem: T): Unit =
    if topIndex >= capacity - 1 then
      throw new RuntimeException("Stack Overflow")
    topIndex += 1
    arr(topIndex) = elem

  def pop(): T =
    if isEmpty then
      throw new RuntimeException("Stack Underflow")
    val elem = arr(topIndex)
    topIndex -= 1
    elem.asInstanceOf[T]

  def top: T =
    if isEmpty then
      throw new RuntimeException("Stack is empty")
    arr(topIndex).asInstanceOf[T]

  def size: Int = topIndex + 1

  def isEmpty: Boolean = topIndex == -1

  override def toString: String =
    val elems = for i <- 0 to topIndex yield arr(i).toString
    elems.mkString("Stack(", ", ", ")")

```
12.10.25, 23:08 - sammy: ich weiß das is jzt spät aber das is stackarray
12.10.25, 23:10 - Ole: alles gut vielen dank
12.10.25, 23:11 - sammy: bist du auch noch wach
12.10.25, 23:11 - sammy: wie klappt inf so?
12.10.25, 23:11 - Ole: ja ich muss noch das mit den bäumen lernen dazu hab ich noch gar nichts gemacht
12.10.25, 23:12 - Ole: das meiste hab ich eig ganz gut verstanden aber ich finde es trotzdem schwer zu lernen
12.10.25, 23:12 - sammy: ja true
12.10.25, 23:12 - sammy: entweder man weiß es auswendig oder nich
12.10.25, 23:13 - sammy: oder so ka
12.10.25, 23:13 - Ole: wie läufts bei dir? hast du viel gemach?
12.10.25, 23:13 - sammy: ne actually nich insgesamt  prob da mit dir am freitag und dann heute so ca 50 min
12.10.25, 23:13 - sammy: das schreib ich mir vielleicht noch ab, könnte hilfreich sein
12.10.25, 23:14 - Ole: ja stimmt
12.10.25, 23:14 - Ole: echt nur so wenig? aber du hast das im unterricht ja eig immer gut verstanden und musstest wahrscheinlich nur wiederholen oder?
12.10.25, 23:15 - sammy: hauptsächlich weil ich halt abgelenkt war und keine lust hatte
12.10.25, 23:15 - Ole: same, ich hatte vor viel mehr zu lernen
12.10.25, 23:15 - sammy: aber ne schon mehr weil das war zum teil vor den ferien und zum anderen hab ich auch nicht immer zugehört
12.10.25, 23:16 - Ole: achso
16.10.25, 16:36 - sammy: Hey, haben wir eigentlich etwas in Geschichte auf?
16.10.25, 20:45 - Ole: Ich glaub nicht aber bin mir nicht sicher
16.10.25, 20:52 - sammy: ah okay danke
24.10.25, 21:56 - sammy: Was machst du eig so in den Ferien
25.10.25, 02:54 - Ole: Eig nix, bin zuhause. Was machst du?
25.10.25, 13:55 - sammy: Auch nich viel, eig war der plan mit meinem Eltern zu meiner oma zufahren, aber die krank und deswegen nicht und so wahrscheinlich nur zuhause
25.10.25, 18:08 - Ole: Schade, hoffentlih geht es deiner oma bald besser
25.10.25, 18:19 - sammy: Ja schon, danke <Diese Nachricht wurde bearbeitet.>
25.10.25, 22:32 - Ole: Das freut mich
27.10.25, 23:40 - sammy: Hast du mal MMOs gespielt? <Diese Nachricht wurde bearbeitet.>
28.10.25, 04:25 - Ole: Bestimmt, aber keine ahnung was alles dazuzählt
28.10.25, 13:29 - sammy: hmmm true gibt inzwischen Ultra viel was sich irgendwie verbindet und dann mischt
"#;

#[bench]
fn bench_long_msg(b: &mut Bencher) {
    b.iter(|| parsetxt(TXT.to_string()));
}
