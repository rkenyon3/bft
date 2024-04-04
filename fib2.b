>++++++++++>+>+                      Initialize: linefeed=10, A=1, T=1, B=0.
[                                    Main loop: each iteration outputs one Fibonacci number from B and computes the next one (A+B).
    [                                Output loop: outputs one digit per iteration
        +++++[>++++++++<-]           Increase B digit by 48 for visible output (ASCII codes for the visible digit characters '0' through '9' are 48 through 57)
        >.<                          Output ASCII for this digit of B.
        ++++++[>--------<-]          Decrease B by 48 to restore it to range 0-9.
        +<<<                         Restore T=1, and go left to the next T marker, if any.
    ]                                End of output loop; go back to output next digit, or exit.
    >.>>                             All digits of B have been output. Output linefeed (ASCII code 10) and go back to the leftmost T marker.
    [                                Update loop: each iteration updates one digit of A and B (setting A to B and B to A+B, with carries)
        [-]                          Clear T to 0 (it was either 1 or 2 before)
        <[>+<-]                      Move this digit of A to empty T cell for the moment.
        >>[<<+>+>-]                  Set A cell = B, and T cell = (A+B) (may exceed 9 now, so we may need to carry a 1 next).
        <[>+<-[>+<-[>+<-[>+<-[>+<-[  Start moving sum from T cell to B cell gradually. (If sum was 0-5 we skip forward as soon as we finish moving it.)
            >+<-[>+<-[>+<-[>+<-      Continue moving sum if it was greater than 5 (this line is cases 6-9).
            [                        We enter this loop if sum exceeded 9, so we need to carry a 1.
                >[-]                 Set B cell = 0 (last digit of "10"; this is case 10).
                >+                   Add 1 to the next digit of A (this is a way to carry a 1 into the next digit of the sum, because in the next iteration of the update loop, that digit of A will go into the sum, but will not persist apart from that).
                >+                   Add 1 to the next T marker to ensure it's nonzero, so the update loop will be executed at least once more. (The next T might have been zero if this sum is longer than the old B, and this is the last carry which lengthens B. A+B is one digit longer than B every 4th or 5th Fibonacci number.)
                <<<-                 Decrease the current T by the 1 we just added to the sum. (We're still moving the sum from T cell to B cell gradually; this long case just changed "9" into "10", carrying a 1.)
                [>+<-]               Move the rest of the sum from T cell to B cell. (If sum was 11-19, this sets B cell = 1-9 to match, storing the sum's second digit in B cell by running this loop 1-9 times. This handles cases 11-19.)
            ]]]]]]]]]]               Ends of skip loops for cases where sum was 0-9 and we skipped doing the carry
        +>>>                         Restore marker T=1 and go to next T marker, if any, to update next digit of A and B.
    ]                                End of update loop; go back to loop start and update next digit, or exit loop when all digits have been updated.
    <<<                              Done updating. New sum A+B is in B cells, previous B value is in A cells now. Go back to rightmost T marker.
]                                    End of main loop, which never terminates; go back to start of main loop and output the new sum.