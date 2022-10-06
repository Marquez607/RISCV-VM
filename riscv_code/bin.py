
binary = "other_test.c.xxd"
with open(binary,"rb") as f:
    content = f.read()
    i = 0;
    while i < len(content):
        print(f"{content[i]:02x}") #{content[i+1]:02x}")
        i+=1
        
        

