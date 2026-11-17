"""vsix.py OUT: packs editors/vscode (the LumaScript extension) into OUT, a .vsix that Visual Studio Code installs
(`code --install-extension OUT`; LumaLink's Settings > Add LumaScript to Visual Studio Code). A .vsix is a zip
(Open Packaging Conventions): [Content_Types].xml, extension.vsixmanifest and the extension's files under extension/.
The identity, version, engine and description come from editors/vscode/package.json. make.py is the extension's
own writer and is left out. Same input, same bytes: entries in name order with a fixed time."""
import json
import os
import sys
import zipfile
from xml.sax.saxutils import escape, quoteattr

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "..", "editors", "vscode")
LEFT_OUT = {"make.py"}
TYPES = {".json": "application/json", ".js": "application/javascript", ".vsixmanifest": "text/xml"}
WHEN = (2026, 1, 1, 0, 0, 0)


def files():
    out = []
    for folder, dirs, names in os.walk(SOURCE):
        dirs[:] = sorted(d for d in dirs if not d.startswith(".") and d != "__pycache__")
        for name in sorted(names):
            if name in LEFT_OUT or name.startswith("."):
                continue
            path = os.path.join(folder, name)
            out.append((os.path.relpath(path, SOURCE).replace(os.sep, "/"), path))
    return out


def manifest(package):
    def attr(v):
        return quoteattr(str(v))
    categories = ",".join(package.get("categories", []))
    return f"""<?xml version="1.0" encoding="utf-8"?>
<PackageManifest Version="2.0.0" xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011" xmlns:d="http://schemas.microsoft.com/developer/vsx-schema-design/2011">
  <Metadata>
    <Identity Language="en-US" Id={attr(package["name"])} Version={attr(package["version"])} Publisher={attr(package["publisher"])} />
    <DisplayName>{escape(package.get("displayName", package["name"]))}</DisplayName>
    <Description xml:space="preserve">{escape(package.get("description", ""))}</Description>
    <Tags>lumascript</Tags>
    <Categories>{escape(categories)}</Categories>
    <GalleryFlags>Public</GalleryFlags>
    <Properties>
      <Property Id="Microsoft.VisualStudio.Code.Engine" Value={attr(package["engines"]["vscode"])} />
      <Property Id="Microsoft.VisualStudio.Code.ExtensionDependencies" Value="" />
      <Property Id="Microsoft.VisualStudio.Code.ExtensionPack" Value="" />
      <Property Id="Microsoft.VisualStudio.Code.ExtensionKind" Value="workspace" />
      <Property Id="Microsoft.VisualStudio.Code.LocalizedLanguages" Value="" />
    </Properties>
  </Metadata>
  <Installation>
    <InstallationTarget Id="Microsoft.VisualStudio.Code" />
  </Installation>
  <Dependencies />
  <Assets>
    <Asset Type="Microsoft.VisualStudio.Code.Manifest" Path="extension/package.json" Addressable="true" />
  </Assets>
</PackageManifest>
"""


def content_types(names):
    exts = sorted({os.path.splitext(n)[1] for n in names} | {".vsixmanifest"})
    rows = "".join(f'<Default Extension="{e}" ContentType="{TYPES.get(e, "application/octet-stream")}" />' for e in exts if e)
    return f'<?xml version="1.0" encoding="utf-8"?>\n<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">{rows}</Types>\n'


def put(z, name, data):
    info = zipfile.ZipInfo(name, WHEN)
    info.compress_type = zipfile.ZIP_DEFLATED
    info.external_attr = 0o644 << 16
    z.writestr(info, data)


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: vsix.py OUT")
    out = sys.argv[1]
    with open(os.path.join(SOURCE, "package.json"), encoding="utf-8") as f:
        package = json.load(f)
    found = files()
    os.makedirs(os.path.dirname(os.path.abspath(out)), exist_ok=True)
    with zipfile.ZipFile(out, "w") as z:
        put(z, "[Content_Types].xml", content_types([n for n, _ in found]))
        put(z, "extension.vsixmanifest", manifest(package))
        for name, path in found:
            with open(path, "rb") as f:
                put(z, "extension/" + name, f.read())
    print(f"{out}: {package['publisher']}.{package['name']} {package['version']}, {len(found)} files")


if __name__ == "__main__":
    main()
