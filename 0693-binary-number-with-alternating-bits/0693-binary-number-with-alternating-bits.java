class Solution {
    public boolean hasAlternatingBits(int n) {
        // Constraint given
        if (n < 1) return false;

        boolean bit =(n % 2 == 0)? false : true;
        n = n / 2;

        while (n > 0) {
            boolean new_bit = (n % 2 == 0)? false : true;
            if (new_bit == bit) {
                return false;
            }
            bit = new_bit;
            n = n/2;
        }
        return true;
    }
}