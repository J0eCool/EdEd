/**IT_START**/

import "env" {
    func log(string);
    func logInt(s32);
}
export {
    func fizzbuzz(s32);
}

/**IT_END**/

void fizzbuzz(int n) {
    for (int i = 1; i <= n; ++i) {
        auto fizz = i % 3 == 0;
        auto buzz = i % 5 == 0;
        if (fizz && buzz) {
            log("FizzBuzz");
        } else if (fizz) {
            log("fizz");
        } else if (buzz) {
            log("buzz");
        } else {
            logInt(i);
        }
    }
}
