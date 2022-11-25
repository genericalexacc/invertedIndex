use std::collections::HashSet;

// Make a set of stop words from string (NLTK stop words).
lazy_static! {
	pub static ref STOP_WORDS_SET: HashSet<String> =
		STOP_WORDS
			.split_whitespace()
			.fold(HashSet::new(), |mut hset, stop_word| {
				hset.insert(stop_word.to_string());
				hset
			});
}

pub static STOP_WORDS: &str = "i
me
my
myself
we
our
ours
ourselves
you
your
yours
yourself
yourselves
he
him
his
himself
she
her
hers
herself
it
its
itself
they
them
their
theirs
themselves
what
which
who
whom
this
that
these
those
am
is
are
was
were
be
been
being
have
has
had
having
do
does
did
doing
a
an
the
and
but
if
or
because
as
until
while
of
at
by
for
with
about
against
between
into
through
during
before
after
above
below
to
from
up
down
in
out
on
off
over
under
again
further
then
once
here
there
when
where
why
how
all
any
both
each
few
more
most
other
some
such
no
nor
not
only
own
same
so
than
too
very
s
t
can
will
just
don
should
now";
