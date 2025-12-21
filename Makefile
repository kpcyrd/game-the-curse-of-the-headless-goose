PNG=$(wildcard art/*.png)
RAW=$(PNG:.png=.raw)

all: $(RAW)

art/%.raw: art/%.png
	embedded-mono-img -o $@ $^
