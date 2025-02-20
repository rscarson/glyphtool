import torch
import database
import lstm
import sys

def stdin_nextline():
    try:
        buff = ''
        while True:
            buff += sys.stdin.read(1)
            if buff.endswith('\n'):
                return buff[:-1]
    except KeyboardInterrupt:
        sys.stdout.flush()
        pass


class Arguments:
    def __init__(self):
        self.args = sys.argv[1:]
        self.args.reverse()

    def next_or_default(self, default):
        if len(self.args) == 0:
            return default
        return self.args.pop()

    def next(self):
        if len(self.args) == 0:
            self.die()
        return self.args.pop()
    
    def die(self):
        print("Usage:")
        print("  autophononimubus train")
        print("  autophononimubus predict <word>")
        print("  autophononimubus server")
        print("  autophononimubus embeddings")
        sys.exit(1)

# Create model instance
device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
db = database.Database()
model = lstm.WordModel(db).to(device)

args = Arguments()
match args.next():
    case "train":
        model.do_train(device, 50)
    
    case "predict":
        input_word = args.next()
        model.load()

        predicted_word = model.predict(device, input_word)
        print(predicted_word)

    case "server":
        model.load()

        while True:
            stdin_word = stdin_nextline()
            if stdin_word is None:
                continue

            predicted_word = model.predict(device, stdin_word)
            print(f"Predicted: {predicted_word}", file=sys.stderr)
            print(predicted_word)
            # flush stdout to ensure the output is sent immediately
            sys.stdout.flush()

    case "embeddings":
        model.load()
        model.view_input_embedding()

    case _:
        args.die()

