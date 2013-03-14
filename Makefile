# This is Makefile version 1.2 of 94/11/08 for Streets and Alleys.

prefix=/usr/local
exec_prefix = $(prefix)
bindir = $(exec_prefix)/bin

SHELL=/bin/sh

PROG	= saa
CFLAGS	= -O

LDLIBS	= -lcurses -ltermcap

all:	$(PROG)

$(PROG):	$(PROG).c
	$(CC) $(CFLAGS) -o $(PROG) $(PROG).c $(LDLIBS)

install:	$(PROG)
	cp -p $(PROG) $(bindir)/$(PROG)

uninstall:
	rm $(bindir)/$(PROG)

dist:
	DATE=`date --iso`; \
	find . -name .git -prune -o -print0 \
		| cpio -pmd0 ../$(PROG)-$${DATE}; \
	cd ..; \
	tar czf $(PROG)-$${DATE}.tar.gz $(PROG)-$${DATE}; \
	rm -rf $(PROG)-$${DATE}

clean:
	-rm $(PROG)

.PHONY:	all clean install uninstall dist
