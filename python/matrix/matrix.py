class Matrix:
    def __init__(self, matrix_string):
        self.rows = []
        self.columns = []
        self.read_in_matrix(matrix_string)

    def row(self, index):
        return self.rows[index-1]

    def column(self, index):
        return self.columns[index-1]

    def read_in_matrix(self, matrix_string):
        rows = matrix_string.split('\n')
        self.rows = [[] for i in range(len(rows))]
        self.columns = [[] for i in range(len(rows[0].split()))]
        for row_i in range(len(rows)):
            row_strs = rows[row_i].split()
            for i in range(len(row_strs)):
                num = int(row_strs[i])
                self.rows[row_i].append(num)
                self.columns[i].append(num)
