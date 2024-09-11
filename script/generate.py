import bs4
import requests


ORACLE_JVM_INSTR_SET = "https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-6.html"


def find(attr, tag, elt):
    return tag if attr in tag.attrs and elt in tag["class"] else None


def camel_name(name):
    return name[0].lower() + name[1:]


def get_decl(name: str, opcode):
    if "<" in name:
        i = name.index("<")
        name = name[: i - 1]
        return """fn %sN(index: u8) -> u8 {
        return %s + index;\n}
        """ % (name, opcode)
    else:
        return """const %s: u8 = %s;""" % (name.upper(), opcode)


def replace_name(name: str):
    if "<" in name:
        i = name.index("<")
        k = name.index(">")
        n = name[i + 1 : k].upper()
        return name[: i - 1] + n
    else:
        return name


def generate_enum(enum_name: str, names):
    res = f"pub enum {enum_name} {{\n"
    for name, value, comment in names:
        res += "\t"
        args = ", ".join("u8" for _ in range(value))
        if args != "":
            res += f"{name}({args})"
        else:
            res += f"{name}"
        res += f", {comment}\n"
    res += "}\n"
    return res


body = requests.get(ORACLE_JVM_INSTR_SET)
bs = bs4.BeautifulSoup(body.text, "html.parser")

divs = bs.find_all("div")
res = []  # List of tuples (opcode, mnemonic)
for div in divs:
    elt = find("class", div, "section-execution")
    if elt is not None:
        res.append(elt)

data = []
code = ""
for elt in res:
    d = {}
    forms = elt.find("div", title="Forms")
    childs = list(forms.children)
    norm = childs[3].text.split("(")
    if len(norm) >= 2:
        d["opcode"] = norm[1].split(")")[0]

    format = elt.find("div", title="Format")
    if format is None:
        continue
    litterallayout = format.find("div", {"class": "literallayout"})
    litterallayout = list(litterallayout.children)
    litterallayout = list(
        filter(lambda x: x.text != "\n", list(litterallayout[1].children)[2:])
    )
    args = []
    for arg in litterallayout:
        if arg.text in [""]:
            continue
        args.append(arg.text)
    nb_args = len(args) - 1
    comment = f"// {" ".join(args)}"
    if "opcode" in d:
        code += get_decl(args[0], d["opcode"]) + "\n"
    name = replace_name(args[0])
    data.append((name, nb_args, comment))

    # command = list(list(litterallayout.children)[0].children)[0].text
    # print(command)

print(code + generate_enum("Opcode", data))
