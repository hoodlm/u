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

  It 'can be used multiple times in one line'
    echo '0 {10} + {6} - STDOUT;' >> $program
    When call $U_INTERPRETER $program

    The status should be success
    The first line of stdout should eq '4'
  End
End
