cd "$(dirname "$0")"

for f in ./*.svg; do
    svg2pts "$f" "${f%.svg}.pts" -d 3
done
