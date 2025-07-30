package main

import (
    "bufio"
    "encoding/json"
    "fmt"
    "html/template"
    "os"
    "path/filepath"
    "regexp"
)

type Message struct {
    Date   string `json:"date"`
    Time   string `json:"time"`
    Sender string `json:"sender"`
    Msg    string `json:"msg"`
}

var msgPattern = regexp.MustCompile(`^(\d{2}\.\d{2}\.\d{2}), (\d{2}:\d{2}) - (.*?): (.*)$`)
var ExeDir string

func parse(filePath string) {
    exePath, err := os.Executable()
    if err != nil {
        fmt.Printf("unable to get executable path: %v\n", err)
        return
    }
    ExeDir = filepath.Dir(exePath)

    file, err := os.Open(filePath)
    if err != nil {
        fmt.Println("Fehler beim Öffnen der Datei:", err)
        return
    }
    defer file.Close()

    var messages []Message
    scanner := bufio.NewScanner(file)

    for scanner.Scan() {
        line := scanner.Text()
        matches := msgPattern.FindStringSubmatch(line)

        if len(matches) == 5 {
            msg := Message{
                Date:   matches[1],
                Time:   matches[2],
                Sender: matches[3],
                Msg:    matches[4],
            }
            messages = append(messages, msg)
        } else {
            fmt.Println("Nicht erkannt oder Fortsetzung:", line)
        }
    }

    if err := scanner.Err(); err != nil {
        fmt.Println("Fehler beim Lesen der Datei:", err)
        return
    }

    // JSON vorbereiten (wird ins HTML eingebettet)
    jsonData, err := json.Marshal(messages)
    if err != nil {
        panic(err)
    }

    // Template laden
    tmplPath := filepath.Join(ExeDir, "out", "template.htm")
    tmpl, err := template.ParseFiles(tmplPath)
    if err != nil {
        panic(err)
    }

    os.MkdirAll("output", os.ModePerm)

    outFile, err := os.Create("output/output.html")
    if err != nil {
        panic(err)
    }
    defer outFile.Close()

    // Template mit JSON füttern
    err = tmpl.Execute(outFile, template.JS(jsonData))
    if err != nil {
        panic(err)
    }

    fmt.Println("✅ HTML-Datei erfolgreich erstellt: output/output.html")
}
