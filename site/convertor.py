input = """| Mass/kg | Tensile Force/N | Ruler reading/mm | Extension/mm | Notes                                                 |
| ------- | --------------- | ---------------- | ------------ | ----------------------------------------------------- |
| 0.1     | 1               | 49.8             | %            | Alex Measured                                         |
| 0.2     | 2               | 49.6             | %            | Alex Measured                                         |
| 0.3     | 3               | 49.5             | %            | Alex Measured                                         |
| 0.4     | 4               | 49.4             | %            | Alex Measured                                         |
| 0.5     | 5               | 49.2             | %            | Alex Measured                                         |
| 0.6     | 6               | 49.2             | %            | Alex Measured                                         |
| 0.7     | 7               | 49.1             | %            | Alex Measured                                         |
| 0.8     | 8               | 49.1             | %            | Alex Measured                                         |
| 0.9     | 9               | 49.0             | %            | Alex Measured                                         |
| 1.0     | 10              | 48.9             | %            | Alex Measured                                         |
| 1.1     | 11              | 48.9             | %            | Ethan Measured                                        |
| 1.2     | 12              | 48.8             | %            | Ethan Measured                                        |
| 1.3     | 13              | 48.7             | %            | Ethan Measured                                        |
| 1.4     | 14              | 48.1             | %            | Someone knocked it - Moved down a lot. Ethan Measured |
| 1.5     | 15              | 48.0             | %            | Caldora Measured                                      |
| 1.6     | 16              | 48.0             | %            | Jack Measured                                         |
| 1.7     | 17              | 47.9             | %            | Liam Measured                                         |
| 1.8     | 18              | 47.9             | %            | Ethan Measured                                        |
| 1.9     | 19              | 47.8             | %            | Ethan Measured                                        |
| 2.0     | 20              | 47.6             | %            | Caldora Measured                                      |
| 2.1     | 21              | 47.7             | %            | Alex Measured                                         |
| 2.2     | 22              | 47.6             | %            | Jack touched wire; Ethan measured                     |
| 2.3     | 23              | 47.5             | %            | Ethan measured                                        |
| 2.4     | 24              | 46.5             | %            | Ethan measured; Ethan dropped weights                 |
| 2.5     | 25              | 46.2             | %            | Ethan measured                                        |
| 2.6     | 26              | 45.9             | %            | Ethan measured                                        |
| 2.7     | 27              | 44.9             | %            | Ethan measured                                        |
| 2.8     | 28              | 43.0             | %            | Ethan measured                                        |
| 2.9     | 29              | 41.1             | %            | Ethan measured; Continuous movement                   |
| 3.0     | 30              | 39.2             | %            | Ethan measured                                        |
| 3.1     | 31              | 36.8             | %            | Ethan measured                                        |
| 3.2     | 32              | 33.2             | %            | Ethan measured                                        |
| 3.3     | 33              | 30.1             | %            | Long delay; Ethan measured                            |
| 3.4     | 34              | 27.9             | %            | Ethan measured                                        |
| 3.5     | 35              | 24.6             | %            | Ethan measured                                        |
| 3.6     | 36              |                  | %            | Hit the floor                                         |"""


out = ""
for i in input.split("\n"):
    if "Ruler reading/mm" in i or "-----" in i:
        out += i + "\n"
        continue
    
    sections = i.split("|")[1:-1]
    sections = [x.strip() for x in sections]
    startingval = 49.8 * 100
    if sections[2] == "":
        extension = "N/A"
    else:
        extension = startingval - int(float(sections[2]) * 100)
        extension /= 100
    sections[3] = str(extension)
    print(sections)

    output = "|" + "|".join(sections) + "|"
    out += output + "\n"

print(out)