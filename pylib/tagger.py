from nltk import word_tokenize
import nltk 
import csv 
import sys 

texts = "And now for something completely different"
if len(sys.argv) > 1:
    path = sys.argv[1]
    with open(path, encoding='utf-8') as f:
        texts = f.read() 

        f.close()

texts = word_tokenize(texts)
with open("target/mid.csv", "w", newline = '') as f:
    cw = csv.writer(f)

    for text in nltk.pos_tag(texts):
        cw.writerow([str(text[0]), str(text[1])])
    f.close()