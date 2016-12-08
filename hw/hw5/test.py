def ret_suffix_array(word):
    ret = []
    for i in range(len(word)):
        ret.append(word[i:])
    # ret.sort()
    return ret

print(ret_suffix_array('mississippi'))