def to_rna(dna_strand):
    rna_converter = {"G": "C", "C": "G", "T": "A", "A": "U"}
    try:
        return "".join([rna_converter[d] for d in dna_strand])
    except Exception as e:
        raise ValueError("Invalid input")
