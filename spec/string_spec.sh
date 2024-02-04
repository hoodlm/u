#shellcheck shell=sh
Describe 'strings'
  Describe 'lowercase'
    It 'can be incremented'
      echo '"ab" + STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'bc'
      The status should be success
    End

    It 'can be incremented multiple times'
      echo '"ab" ++ STDOUT +++ STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be success
      The first line of stdout should eq 'cd'
      The second line of stdout should eq 'fg'
    End

    It 'can be decremented'
      echo '"cf" - STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'be'
      The status should be success
    End

    It 'can be decremented multiple times'
      echo '"pm" -- STDOUT ---- STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be success
      The first line of stdout should eq 'nk'
      The second line of stdout should eq 'jg'
    End

    It 'can be incremented and overflow'
      echo '"za" + STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'ab'
      The status should be success
    End

    It 'can be decremented and underflow'
      echo '"abc" - STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'zab'
      The status should be success
    End
  End

  Describe 'uppercase'
    It 'can be incremented'
      echo '"GH" + STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'HI'
      The status should be success
    End

    It 'can be decremented'
      echo '"XYZ" - STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'WXY'
      The status should be success
    End

    It 'can be incremented and overflow'
      echo '"XYZ" + STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'YZA'
      The status should be success
    End

    It 'can be decremented and underflow'
      echo '"DAB" - STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'CZA'
      The status should be success
    End
  End

  It 'can be rot13-ciphered with decrement'
    echo '"HELLO world" ----- ----- --- STDOUT;' >> $program
    When call $U_INTERPRETER $program
    The status should be success
    The stdout should eq "URYYB jbeyq"
  End

  It 'can be rot13-ciphered with increment'
    echo '"HELLO world" +++++ +++++ +++ STDOUT;' >> $program
    When call $U_INTERPRETER $program
    The status should be success
    The stdout should eq "URYYB jbeyq"
  End

  Describe 'Strings with whitespace'
    It 'can be printed to STDOUT'
      echo '"A B C D" STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be success
      The stdout should eq "A B C D"
    End

    It 'can be incremented'
      echo '"A B C D" +++ STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be success
      The stdout should eq "D E F G"
    End

    It 'can be decremented'
      echo '" A  Z " -- STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be success
      The stdout should eq " Y  X "
    End
  End

  It 'can be one character long'
    echo '"A" + STDOUT;' >> $program
    When call $U_INTERPRETER $program
    The status should be success
    The stdout should eq "B"
  End
End
