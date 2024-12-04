import re
with open("input.txt", "r") as f:
    data = f.read()

mul_rgx = re.compile(r"mul\((\d{1,3}),(\d{1,3})\)")
mulsum = 0
for (op1, op2) in mul_rgx.findall(data):
    print(f"+ ({op1} * {op2})")
    mulsum += (int(op1) * int(op2))
print(f"= {mulsum}")
