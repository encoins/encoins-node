all: main

main:
	pdflatex main.tex

scratch:
	pdflatex main.tex
	bibtex main
	pdflatex main.tex
	pdflatex main.tex
clean:
	rm -f *.log *.aux *.out *.toc *.dvi *.bbl *.blg

mrproper: clean
	rm *.pdf

