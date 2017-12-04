#!/usr/bin/env perl6

my $testInput = "5 1 9 5\n7 5 3\n2 4 6 8";

sub minmaxdiff(@nums) {
  max(@nums) - min(@nums);
}

sub tonumeric($i) {
$i.split("\n").map({$_.split(/\h/).map({:10($_)})});
}
my @numericRows = tonumeric($testInput);

say @numericRows.map(&minmaxdiff).sum;

say tonumeric("input2.txt".IO.slurp.chomp).map(&minmaxdiff).sum;
