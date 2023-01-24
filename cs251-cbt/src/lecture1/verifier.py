#!python3

import sys
import hashlib
from base64 import b64encode, b64decode
from collections import deque
import re

prooffile = "proof.txt"    # File containing the Merkle proof to verify.   
                           # Change this to load a different file. 

MAXHEIGHT = 20             # Max height of Merkle tree

class MerkleProof:
    def __init__(self, leaf, pos, path):
        self.leaf = leaf  # data of leaf being checked
        self.pos  = pos   # the position in the tree of the leaf being checked
        self.path = path  # the path of hashes, from bottom to the top of tree


def hash_leaf(leaf):
    """hash a leaf value."""
    sha256 = hashlib.sha256()
    sha256.update(b"leaf:")   # hash prefix for a leaf
    sha256.update(leaf)
    return sha256.digest()

def hash_internal_node(left, right):
    """hash an internal node."""
    sha256 = hashlib.sha256()
    sha256.update(b"node:")   # hash prefix for an internal node
    sha256.update(left)
    sha256.update(right)
    return sha256.digest()

##  The prefixes in the two functions above are a security measure.
##  They provide domain separation, meaning that the domain of a leaf hash
##  is seperated from the domain of an internal node hash.
##  This ensures that the verifier cannot mistake a leaf hash for 
##  an internal node hash, and vice versa. 


def compute_merkle_root_from_proof(proof: MerkleProof):
    """computes a root from the given leaf and Merkle proof."""
    pos = proof.pos
    path = deque(proof.path)
    root = hash_leaf(proof.leaf)
    while path:
        if pos % 2 == 0:
            left, right = root, path.popleft()
        else:
            left, right = path.popleft(), root
        root = hash_internal_node(left, right)
        pos >>= 1
    return root   # return the computed root


### Helper function
def read_proof(filename):
    """Read the leaf data, position of leaf, and Merkle proof from file."""
    fp = open(filename, "r")
    pos  = int(re.search('(\d*)$', fp.readline()).group(1))
    leaf = re.search('\"(.*)\"',fp.readline()).group(1).encode()
    fp.readline()
    path = fp.readlines()
    for i in range(len(path)):
        path[i] = b64decode((path[i])[2:])
    fp.close()
    return MerkleProof(leaf=leaf, pos=pos, path=path)


### Main program
if __name__ == "__main__":

    # This is the hardcoded root of Merkle tree 
    root = b64decode("1qIbsvuF6FrhNjMD4p06srUye6G4FfFINDDkNfKUpTs=")
    print('\nHave hardcoded root of committed Merkle tree.')

    # Read (leaf data, position of leaf, and proof) from file
    proof = read_proof(prooffile)

    # Verify that proof length is correct
    height = len(proof.path)
    assert height < MAXHEIGHT, "Proof is too long"

    # Verify proof
    computedRoot = compute_merkle_root_from_proof(proof)
    assert root == computedRoot, "Verify failed"

    print('I verified the Merkle proof: leaf #{} in the committed tree is "{}".\n'.format(proof.pos,proof.leaf.decode("utf-8")))
    sys.exit(0)



