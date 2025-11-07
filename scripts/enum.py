with open("symbols.txt", "r") as f:
    symbols = f.read().split(", ")

with open("enum.rs", "w") as f:
    f.write("pub enum Symbol {\n")
    for symbol in symbols:
        value = symbol.split("_")
        fst = value[0].capitalize()
        snd = value[1].capitalize() if len(value) > 1 else ""
        f.write(f"    /// {symbol}\n")
        f.write(f"    #[serde(rename = \"{symbol}\")]\n")
        if snd:
            f.write(f"    {fst}{snd},\n")
        else:
            f.write(f"    {fst},\n")
    f.write("}\n")