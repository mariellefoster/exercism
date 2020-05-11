# Codon   Protein

# UGU, UGC    Cysteine
# UGG Tryptophan
# UAA, UAG, UGA   STOP

codon = {
    "AUG" : "Methionine",
    "UUU" : "Phenylalanine",
    "UUC" : "Phenylalanine",
    "UUA" : "Leucine",
    "UUG" : "Leucine",
    "UCU" : "Serine",
    "UCC" : "Serine",
    "UCA" : "Serine",
    "UCG" : "Serine",
    "UAU" : "Tyrosine",
    "UAC" : "Tyrosine",
    "UGU" : "Cysteine",
    "UGC" : "Cysteine",
    "UGG" : "Tryptophan",
    "UAA" : "STOP",
    "UAG" : "STOP",
    "UGA" : "STOP",
}

def proteins(strand):
    transl = []
    for i in range(int(len(strand)/3)):
        q = i*3
        c = strand[q:q+3]
        if c not in codon:
            raise Exception("String poorly formed")
        if codon[c] == "STOP":
            return transl
        transl.append(codon[c])
    return transl

    # [ for i in len(strand)/3]
