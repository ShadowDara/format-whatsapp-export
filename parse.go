package main

import (
    "bufio"
    "fmt"
    "html/template"
    "os"
    "regexp"
	"path/filepath"
)

type Message struct {
    Date    string
    Time    string
    Sender  string
    Content string
}

var msgPattern = regexp.MustCompile(`^(\d{2}\.\d{2}\.\d{2}), (\d{2}:\d{2}) - (.*?): (.*)$`)

var ExeDir string

func parse(filePath string) {
	exePath, err := os.Executable()
    if err != nil {
        fmt.Errorf("unable to get executable path: %w", err)
    }

    ExeDir = filepath.Dir(exePath)

    file, err := os.Open(filePath)
    if err != nil {
        fmt.Println("Fehler beim Öffnen der Datei:", err)
        return
    }
    defer file.Close()

    // HTML-Template laden
    tmplPath := filepath.Join(ExeDir, "static", "template.html")
	tmpl, err := template.ParseFiles(tmplPath)
    if err != nil {
        panic(err)
    }

    // Nachrichten-Array vorbereiten
    var lines []Message

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        line := scanner.Text()

        matches := msgPattern.FindStringSubmatch(line)
        if len(matches) == 5 {
            msg := Message{
                Date:    matches[1],
                Time:    matches[2],
                Sender:  matches[3],
                Content: matches[4],
            }

            lines = append(lines, msg)
        } else {
            fmt.Println("Nicht erkannt oder Fortsetzung:", line)
        }
    }

    if err := scanner.Err(); err != nil {
        fmt.Println("Fehler beim Lesen der Datei:", err)
    }

    // Output-Verzeichnis sicherstellen
    os.MkdirAll("output", os.ModePerm)

    outFile, err := os.Create("output/output.html")
    if err != nil {
        panic(err)
    }
    defer outFile.Close()

    // Template ausführen
    err = tmpl.Execute(outFile, lines)
    if err != nil {
        panic(err)
    }

    fmt.Println("✅ HTML-Datei erfolgreich erstellt: output/output.html")
}
