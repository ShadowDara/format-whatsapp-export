package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"github.com/sqweek/dialog"
	"log"
	"os"
	"path/filepath"
)

//go:embed template.htm
var htmltemplate string

func main() {
	fmt.Println("Whatsapp Chat Export Formatter\n")

	fmt.Println("Searching for the txt File automaticly")
	txtFiles := searchTxt()

	var empty = true
	for _, file := range txtFiles {
		parse(file)
		empty = false
	}

	// When txt file was found automaticly, the user should select the path
	if empty {
		fmt.Println("txt File not found")
		fmt.Println("[PRESS ENTER]")
		bufio.NewReader(os.Stdin).ReadString('\n')

		fmt.Println("\nPlease select the location of your Whatsapp Export")
		fmt.Println("[PRESS ENTER]")
		bufio.NewReader(os.Stdin).ReadString('\n')

		dir, err := dialog.Directory().Title("Select the Folder").Browse()
		if err != nil {
			log.Fatal(err)
		}

		fmt.Println("Selected Folder:", dir)
		fmt.Println("Trying to use this Folder...")
		txtFiles := searchTxt()

		var empty2 = true
		for _, file := range txtFiles {
			parse(file)
			empty2 = false
		}

        if (empty2) {
            fmt.Println("This did not work!")
        }
	}
}

// bessere Methode, nach über geordneten ordner suchen und dann
// datei mit dem gleichen namen wie der ordner suchen!
func searchTxt() []string {
	// Aktuelles Arbeitsverzeichnis ermitteln
	dir, err := os.Getwd()
	if err != nil {
		fmt.Println("Error while trying to find the executing folder:", err)
		return []string{}
	}

	// Pfad mit *.txt-Muster bauen
	pattern := filepath.Join(dir, "*.txt")

	// Nach Dateien suchen
	files, err := filepath.Glob(pattern)
	if err != nil {
		fmt.Println("Error while searching:", err)
		return []string{}
	}

	// Ausgabe der gefundenen Dateien
	for _, file := range files {
		fmt.Println("Found:", file)
	}

	return files
}
