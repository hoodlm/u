#shellcheck shell=sh
Describe 'letters'
  Describe 'lowercase'
    It 'can be incremented'
      echo "'a' + + STDOUT;" >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'c'
      The status should be success
    End

    It 'can be decremented'
      echo "'d' - - STDOUT;" >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'b'
      The status should be success
    End

    It 'can be incremented and overflow'
      echo "'z' + STDOUT;" >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'a'
      The status should be success
    End

    It 'can be decremented and underflow'
      echo "'a' - STDOUT;" >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'z'
      The status should be success
    End
  End

  Describe 'uppercase'
    It 'can be incremented'
      echo "'G' + + STDOUT;" >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'I'
      The status should be success
    End

    It 'can be decremented'
      echo "'Y' - - STDOUT;" >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'W'
      The status should be success
    End

    It 'can be incremented and overflow'
      echo "'Z' + STDOUT;" >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'A'
      The status should be success
    End

    It 'can be decremented and underflow'
      echo "'A' - STDOUT;" >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'Z'
      The status should be success
    End
  End

  It 'can be rot13-ciphered one character per line'
    echo "'H' - - - - - - - - - - - - - STDOUT;" >> $program
    echo "'E' - - - - - - - - - - - - - STDOUT;" >> $program
    echo "'L' - - - - - - - - - - - - - STDOUT;" >> $program
    echo "'L' - - - - - - - - - - - - - STDOUT;" >> $program
    echo "'O' - - - - - - - - - - - - - STDOUT;" >> $program
    echo "'w' - - - - - - - - - - - - - STDOUT;" >> $program
    echo "'o' - - - - - - - - - - - - - STDOUT;" >> $program
    echo "'r' - - - - - - - - - - - - - STDOUT;" >> $program
    echo "'l' - - - - - - - - - - - - - STDOUT;" >> $program
    echo "'d' - - - - - - - - - - - - - STDOUT;" >> $program

    When call $U_INTERPRETER $program
    The status should be success
    The stdout lines should eq 10
    The line 1 of stdout should eq 'U'
    The line 2 of stdout should eq 'R'
    The line 3 of stdout should eq 'Y'
    The line 4 of stdout should eq 'Y'
    The line 5 of stdout should eq 'B'
    The line 6 of stdout should eq 'j'
    The line 7 of stdout should eq 'b'
    The line 8 of stdout should eq 'e'
    The line 9 of stdout should eq 'y'
    The line 10 of stdout should eq 'q'
  End
End
