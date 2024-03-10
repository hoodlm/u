#shellcheck shell=sh
Describe '{} repeater'
  It 'can be used to repeat the increment operator'
    echo "5 {3} + STDOUT;" >> $program
    When call $U_INTERPRETER $program

    The stdout should eq '8'
    The status should be success
  End

  It 'can be used to repeat the decrement operator'
    echo "10 {5} - STDOUT;" >> $program
    When call $U_INTERPRETER $program

    The stdout should eq '5'
    The status should be success
  End

  It 'can be used to implement caesar ciphers'
    echo '"Hello world" {13} + STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The stdout should eq 'Uryyb jbeyq'
    The status should be success
  End

  It 'can be used to repeat the STDOUT operator'
    echo '"Hello world" {2} STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The status should be success
    The first line of stdout should eq 'Hello world'
    The second line of stdout should eq 'Hello world'
  End

  It 'can be used with large values'
    echo '5 {100} + STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The status should be success
    The first line of stdout should eq '105'
  End

  It 'can be used with very large values'
    echo '1 {100000} + STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The status should be success
    The first line of stdout should eq '100001'
  End

  It 'can be used with zero'
    echo '1 {0} + STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The status should be success
    The first line of stdout should eq '1'
  End

  It 'can be used with one'
    echo '1 {1} + STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The status should be success
    The first line of stdout should eq '2'
  End

  It 'can be used multiple times in one line'
    echo '0 {10} + {6} - STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The status should be success
    The first line of stdout should eq '4'
  End

  It 'can be used to repeat itself'
    # This is multiplication
    echo '0 {10} {6} + STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The status should be success
    The first line of stdout should eq '60'
  End

  It 'can be used to repeat itself several times'
    # This computes 5 factorial !
    echo '0 {5} {4} {3} {2} {1} + STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The status should be success
    The first line of stdout should eq '120'
  End

  It 'returns a lexer error when used with a negative number'
    echo '0 {-1} + STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The status should be failure
    The first line of stderr should eq 'Lexical analysis failed!'
  End

  It 'returns a syntax error at the end of a line'
    echo '0 {1};' >> $program
    When call $U_INTERPRETER $program

    The status should be failure
    The first line of stderr should eq 'Syntax analysis failed!'
    The line 2 of stderr should include 'Unexpected token'
    The line 2 of stderr should include ';'
  End
End
