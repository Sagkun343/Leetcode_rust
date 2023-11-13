class Solution:
    def sortVowels(self, s: str) -> str:
        vowel_set = {'a', "A", 'E', 'e', 'i', 'I', 'o', 'O', 'u', "U"}
        # vowel_index = []
        vowel_list = []
        res = str()
        v_ptr = 0
        for i in range(len(s)):
            if s[i] in vowel_set:
                # vowel_index.append(i)
                vowel_list.append(s[i])
        vowel_list = sorted(vowel_list)
        for i in range(len(s)):
            if s[i] not in vowel_set:
                res += s[i]
            else:
                res += (vowel_list[v_ptr])
                v_ptr += 1
        return res
        