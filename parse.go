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

func parse(filePath string, output_dir string) {
    fmt.Println("Parsing the data...")

    exePath, err := os.Executable()
    if err != nil {
        fmt.Printf("unable to get executable path: %v\n", err)
        return
    }
    ExeDir = filepath.Dir(exePath)

    file, err := os.Open(filePath)
    if err != nil {
        fmt.Println("Error while opening file:", err)
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
            // to much Terminal Spam with this print function
            //fmt.Println("Nicht erkannt oder Fortsetzung:", line)
        }
    }

    if err := scanner.Err(); err != nil {
        fmt.Println("Error while reading the file:", err)
        return
    }

    // JSON vorbereiten (wird ins HTML eingebettet)
    jsonData, err := json.Marshal(messages)
    if err != nil {
        panic(err)
    }

    // Template laden
    tmpl, err := template.New("example").Parse(htmltemplate)
    if err != nil {
        panic(err)
    }

    os.MkdirAll(output_dir + "/output", os.ModePerm)

    outFile, err := os.Create(output_dir + "/output/output.html")
    if err != nil {
        panic(err)
    }
    defer outFile.Close()

    // Template mit JSON füttern
    err = tmpl.Execute(outFile, template.JS(jsonData))
    if err != nil {
        panic(err)
    }

    //fmt.Println(output_dir)
    fmt.Printf("HTML File created successfully in: %s/output/output.html\n", output_dir)
}
