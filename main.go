package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/sqweek/dialog"
)

//go:embed template.htm
var htmltemplate string

var (
	version   = "dev"
	buildTime = "unknown"
)

func main() {
	fmt.Println("Whatsapp Chat Export Formatter by Shadowdara")
	fmt.Println("Version:", version)
	fmt.Println("Build Time:", buildTime)
	fmt.Println("https://github.com/ShadowDara/format-whatsapp-export")

	fmt.Println("\nFormmating your Whatsapp Chat Export now\n")

	var txtFiles []string
	fmt.Println("Searching for the txt File automatically :)")

	dir, err := os.Getwd()
	if err != nil {
		fmt.Println("Error while trying to find the executing folder:", err)
	} else {
		txtFiles = searchTxt(dir)
	}

	empty := true
	for _, file := range txtFiles {
		parse(file, dir)
		empty = false
	}

	// When txt file was found automaticly, the user should select the path
	if empty {
		fmt.Println("txt File not found :(\n")
		fmt.Println("[PRESS ENTER]")
		bufio.NewReader(os.Stdin).ReadString('\n')

		fmt.Println("Please select the location of your Whatsapp Export\n")
		fmt.Println("[PRESS ENTER]")
		bufio.NewReader(os.Stdin).ReadString('\n')

		// select a folder
		dir, err := dialog.Directory().Title("Select the Folder").Browse()
		if err != nil {
			log.Println(err)

			fmt.Println("Folder Selection cancelled!")
			fmt.Println("For more information, visit the Github Page.")
			fmt.Println("or just restart the program\n")

			fmt.Println("[PRESS ENTER]")
			bufio.NewReader(os.Stdin).ReadString('\n')

			// to exit the program
			log.Fatal("Exit")
		}

		fmt.Println("Selected Folder:", dir)
		fmt.Println("\nTrying to use this Folder...")

		txtFiles2 := searchTxt(dir)

		empty2 := true
		for _, file2 := range txtFiles2 {
			parse(file2, dir)
			empty2 = false
		}

		if empty2 {
			fmt.Println("This did not work!")
		} else {
			fmt.Println("")
		}
	}
}

// bessere Methode, nach über geordneten ordner suchen und dann
// datei mit dem gleichen namen wie der ordner suchen!
func searchTxt(dir string) []string {
	fmt.Println("Searching TXT File")
	// Aktuelles Arbeitsverzeichnis ermitteln

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
