import database
import torch
from torch.utils.data import Dataset, DataLoader
import torch.nn as nn
import torch.nn.functional as F
import torch.optim as optim
from torch.nn.utils.rnn import pad_sequence
from torch.optim.lr_scheduler import StepLR
import matplotlib.pyplot as plt
from sklearn.decomposition import PCA
from sklearn.manifold import TSNE
import pathlib

MAX_WORD_LEN = 50

class WordModel(nn.Module):
    def __init__(self, db, embedding_dim=256, num_filters=128, kernel_size=3, dropout=0.5):
        super(WordModel, self).__init__()
        self.database = db
        self.encoder = self.database.encoding_table
        self.corpus = self.database.all()

        # Embedding layer for input
        self.input_embedding = nn.Embedding(len(self.encoder.data), embedding_dim, padding_idx=0)
        
        self.conv = nn.Conv1d(
            in_channels=embedding_dim, out_channels=num_filters,
            kernel_size=kernel_size, padding=kernel_size // 2
        )

        self.fc = nn.Linear(num_filters, 1)  # Binary classification
        self.dropout = nn.Dropout(0.2)  # Prevent overfitting

        
    def save(self):
        dir = pathlib.Path(__file__).parent.resolve()
        torch.save(self.state_dict(), dir / 'model.pth')

    def load(self):
        dir = pathlib.Path(__file__).parent.resolve()
        self.load_state_dict(torch.load(dir / 'model.pth'))

    def forward(self, x):
        x = x.long()
        x = self.input_embedding(x)  # (batch, seq_len, embedding_dim)
        x = x.permute(0, 2, 1)  # (batch, embedding_dim, seq_len) for CNN
        x = self.conv(x)  # Convolutional layer
        x = x.permute(0, 2, 1)  # Back to (batch, seq_len, features)
        x = self.dropout(x)  

        logits = self.fc(x)  # Final classification
        return logits.squeeze(-1)  # (batch, seq_len)

    def do_train(self, device, epochs = 10):
        super().train()

        # Choose loss function and optimizer
        criterion = nn.BCEWithLogitsLoss()
        optimizer = optim.Adam(self.parameters(), lr=0.001)
        scheduler = StepLR(optimizer, step_size=5, gamma=0.1)  # Every 5 epochs, decay by a factor of 0.1
        early_stopping = EarlyStopping(patience=5, delta=0.001) # Stop training if loss does not improve

        inputs = [x[0] for x in self.corpus]
        outputs = [x[1] for x in self.corpus]
        train_dataset = SequenceDataset(inputs, outputs)
        train_loader = DataLoader(train_dataset, batch_size=2, shuffle=False, collate_fn=collate_fn)
        
        for epoch in range(epochs):
            total_loss = 0
            
            for batch in train_loader:
                inputs, targets = batch  # (batch_size, seq_len), (batch_size, seq_len)
                inputs, targets = inputs.to(device), targets.float().to(device)  # Move to GPU if available
                
                optimizer.zero_grad()
                logits = self(inputs)  # Get raw logits
                loss = criterion(logits, targets.squeeze(-1))  # Remove the extra dimension
                loss.backward()  # Backpropagation
                torch.nn.utils.clip_grad_norm_(self.parameters(), max_norm=1.0)
                optimizer.step()  # Update weights
                
                total_loss += loss.item()
            
            scheduler.step()  # Update the learning rate after each epoch
            print(f"Epoch {epoch+1}, Loss: {total_loss / len(train_loader):.4f}")
            if early_stopping.step(loss.item()):
                print(f"Early stopping at epoch {epoch}")
                break

        self.save()

    def predict(self, device, input):
        self.eval()  # Set the model to evaluation mode
        encoder = self.encoder
        input = encoder.encode_word(input)

        # Encode input word
        input_tensor = torch.tensor(input).unsqueeze(0).to(device)

        with torch.no_grad():
            logits = self(input_tensor)
            probs = torch.sigmoid(logits)  # Convert to probability
            predictions = (probs > 0.5).long()  # Threshold at 0.5
        prediction = predictions.cpu().numpy()

        output = encoder.split_with(input, prediction[0])
        return encoder.decode_word(output)
    
    def view_input_embedding(self):
        encoder = self.encoder
        embedding_weights = self.input_embedding.weight.detach().numpy()

        # Reduce dimensions
        pca = PCA(n_components=2)
        embeddings_2d = pca.fit_transform(embedding_weights)

        # Plot
        plt.figure(figsize=(10, 7))
        plt.scatter(embeddings_2d[:, 0], embeddings_2d[:, 1], alpha=0.5) # Annotate some points

        for idx, symbol in enumerate(encoder.data):
            if idx == 0:
                plt.text(embeddings_2d[0, 0], embeddings_2d[0, 1], "?", fontsize=9)
            else:
                plt.text(embeddings_2d[idx, 0], embeddings_2d[idx, 1], symbol, fontsize=9)

        plt.title("PCA of Input Symbol Embeddings")
        plt.show()

class EarlyStopping:
    def __init__(self, patience=5, delta=0):
        self.patience = patience
        self.delta = delta
        self.best_loss = None
        self.counter = 0

    def step(self, loss):
        if self.best_loss is None:
            self.best_loss = loss
        elif loss > self.best_loss + self.delta:
            self.counter += 1
            if self.counter >= self.patience:
                return True
        else:
            self.best_loss = loss
            self.counter = 0
        return False
    
class SequenceDataset(Dataset):
    def __init__(self, sequences, labels):
        self.sequences = sequences  # List of tokenized sequences (list of lists)
        self.labels = labels  # List of binary labels per sequence

    def __len__(self):
        return len(self.sequences)

    def __getitem__(self, idx):
        input_tensor = torch.tensor(self.sequences[idx], dtype=torch.long)  # Ensure long type for embedding lookup
        label_tensor = torch.tensor(self.labels[idx], dtype=torch.float)  # No unsqueeze
        return input_tensor, label_tensor

def collate_fn(batch):
    sequences, labels = zip(*batch)
    
    # Use clone().detach() instead of torch.tensor() for existing tensors
    sequences = pad_sequence([seq.clone().detach().long() for seq in sequences], batch_first=True, padding_value=0)
    
    # Convert labels properly using clone().detach()
    labels = pad_sequence([seq.clone().detach().float() for seq in labels], batch_first=True, padding_value=0)
    
    return sequences, labels
