package main

import (
    "fmt"
    "os"
    "path/filepath"
	"github.com/shadowdara/checkstaticfiles"
)

func main() {
	shadowdara_checkstaticfiles.Checkfiles(CheckstaticfilesOutputJSONGz, Chechstaticfiles_settings)

    txtFiles := searchTxt()
	for _, file := range txtFiles {
		parse(file)
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
