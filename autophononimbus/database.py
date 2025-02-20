import sqlite3

DEFAULT_PATH = 'phonemes.db'

class Database:
    def __init__(self, path = DEFAULT_PATH):
        self.conn = sqlite3.connect(path)

        # read id/phoneme pairs from 'encoding' and get our encoding table
        cursor = self.conn.cursor()
        cursor.execute('SELECT id, phoneme FROM encoding')
        self.encoding_table = EncodingTable(cursor.fetchall())

    def all(self):
        sep = self.encoding_table.encode('-')
        cursor = self.conn.cursor()
        cursor.execute('SELECT phonemes FROM words')

        inputs = [list(row[0]) for row in cursor.fetchall()]
        corpus = []
        for word in inputs:
            input = [x for x in word if x != sep]
            output = []
            for c in word:
                if c == sep:
                    if len(output) > 0:
                        output[-1] = 1
                else:
                    output.append(0)

            corpus.append((input, output))
        return corpus
        
class EncodingTable:
    # Accepts a [(int, str)] list
    def __init__(self, data):
        data = sorted(data, key = lambda x: x[0])
        self.data = [x[1] for x in data]

    def split_with(self, phonemes, is_boundary):
        sep = self.encode('-')
        buffer = []
        for i in range(len(phonemes)):
            buffer.append(phonemes[i])
            if is_boundary[i] == 1:
                buffer.append(sep)
        return buffer

    def decode(self, index):
        if index not in range(len(self.data)):
            print(f"Warning: Out of range value {index}")
            return self.data[0]
        return self.data[index]
    
    def encode(self, phoneme):
        index = self.data.index(phoneme)
        if index == -1:
            return 0
        return index
    
    def decode_word(self, word):
        return ''.join(self.decode(x) for x in word)
    
    def encode_word(self, word):
        indices = []
        buffer = word
        while buffer:
            found = False
            for i, phoneme in enumerate(self.data):
                if buffer.startswith(phoneme):
                    indices.append(i)
                    buffer = buffer[len(phoneme):]
                    found = True
                    break
            if not found:
                indices.append(0)
                buffer = buffer[1:]
        return indices

class AsciiEncoder:
    def __init__(self):
        self.data = ['\0']
        self.data.extend([chr(x) for x in range(ord('a'), ord('z')+1)])
        self.data.extend(['E', 'O', 'A', '\''])

    def decode(self, index):
        if index not in range(len(self.data)):
            return self.data[0]
        return self.data[index]
    
    def encode(self, phoneme):
        index = self.data.index(phoneme)
        if index == -1:
            return 0
        return index
    
    def decode_word(self, word):
        return ''.join(self.decode(x) for x in word)
    
    def encode_word(self, word):
        return [self.encode(x) for x in word]