#!/usr/bin/env perl6

my $inputText = slurp "input1.txt";

sub parse_input($inputText) {
  $inputText.split("", :skip-empty).grep(/\d+/).map({:10($_)});
}
my @numbers = parse_input($inputText);


sub first_part(@numbers) {
  my @pairs = @numbers.rotor(2 => -1, :partial);


  @pairs[*-1] = (@pairs[*-1][0], @numbers[0]);

  @pairs.grep({$_[0] == $_[1]}).map(*[0]).sum;
}

say first_part(@numbers);

sub second_part(@numbers) {
  my $halfway = @numbers.elems / 2;
  my @rotated = @numbers.rotate($halfway);

  my @pairs = zip(@numbers, @rotated);

  @pairs.grep({$_[0] == $_[1]}).map(*[0]).sum;
}

say second_part(@numbers);
