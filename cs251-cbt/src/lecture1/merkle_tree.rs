use std::{
    collections::VecDeque,
    fs,
    io::{Read, Write},
};

use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const PROOFFILE: &str = "data/lecture1/proof.txt"; // File where Merkle proof will be written.
const SERDE_PROOF_FILE: &str = "data/lecture1/merkle-proof-serde.json";

// Max height of Merkle tree
const MAX_HEIGHT: usize = 20;

#[derive(Clone, Debug)]
pub struct MerkleProof {
    /// data of leaf being checked
    pub leaf: Vec<u8>,
    /// the position in the tree of the leaf being checked
    pub pos: usize,
    /// the path of hashes, from bottom to the top of tree
    pub path: Vec<Vec<u8>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MerkleProofSerde {
    /// data of leaf being checked
    leaf: String,
    /// the position in the tree of the leaf being checked
    pos: usize,
    /// the path of hashes, from bottom to the top of tree
    path: Vec<String>,
}

impl TryFrom<MerkleProof> for MerkleProofSerde {
    type Error = anyhow::Error;

    fn try_from(value: MerkleProof) -> Result<Self, Self::Error> {
        Ok(MerkleProofSerde {
            leaf: String::from_utf8(value.leaf)?,
            pos: value.pos,
            path: value.path.into_iter().map(base64::encode).collect(),
        })
    }
}

impl TryFrom<MerkleProofSerde> for MerkleProof {
    type Error = anyhow::Error;
    fn try_from(value: MerkleProofSerde) -> Result<Self, Self::Error> {
        Ok(Self {
            leaf: value.leaf.as_bytes().to_vec(),
            pos: value.pos,
            path: value
                .path
                .into_iter()
                .map(base64::decode)
                .collect::<Result<Vec<Vec<u8>>, _>>()?,
        })
    }
}

impl MerkleProof {
    pub fn new(leaf: Vec<u8>, pos: usize, path: Vec<Vec<u8>>) -> MerkleProof {
        Self { leaf, pos, path }
    }

    pub fn write_proof(&self, file_name: String) -> anyhow::Result<()> {
        let mut fp = if let Ok(fp) = fs::File::open(file_name.clone()) {
            fp
        } else {
            fs::File::create(file_name)?
        };

        fp.write_all(format!("leaf position: {}\n", self.pos).as_bytes())?;
        fp.write_all(
            format!(
                "leaf value: \"{}\"\n",
                String::from_utf8(self.leaf.clone())?
            )
            .as_bytes(),
        )?;
        fp.write_all("Hash values in proof:\n".to_string().as_bytes())?;
        for path in self.path.clone() {
            fp.write_all(format!("  {}\n", base64::encode(path)).as_bytes())?;
        }

        Ok(())
    }

    // ### Helper function
    /// Read the leaf data, position of leaf, and Merkle proof from file.
    pub fn read_proof(file_name: String) -> anyhow::Result<MerkleProof> {
        let mut fp = fs::File::open(file_name)?;
        let mut contents = String::new();
        fp.read_to_string(&mut contents)?;

        let serde_proof: MerkleProofSerde = serde_json::from_str(&contents)?;

        let merkle_proof = MerkleProof::try_from(serde_proof)?;

        Ok(merkle_proof)
    }
}

/// hash a leaf value.
pub fn hash_leaf(leaf: Vec<u8>) -> Vec<u8> {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // hash prefix for a leaf
    hasher.update(b"leaf:");

    // write input message
    hasher.update(leaf);

    // read hash digest and consume hasher
    let result = hasher.finalize();

    result.to_vec()
}

/// hash an internal node.
pub fn hash_internal_node(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // hash prefix for an internal node
    hasher.update(b"node:");

    // write input message
    hasher.update(left);
    hasher.update(right);

    // read hash digest and consume hasher
    let result = hasher.finalize();

    result.to_vec()
}

//  The prefixes in the two functions above are a security measure.
//  They provide domain separation, meaning that the domain of a leaf hash
//  is seperated from the domain of an internal node hash.
// This ensures that the verifier cannot mistake a leaf hash for
// an internal node hash, and vice versa.

/// Takes as input a list of leaves and a leaf position (pos).
/// Returns the Merkle proof for the leaf at pos.
pub fn gen_merkle_proof(leaves: Vec<Vec<u8>>, pos: usize) -> Vec<Vec<u8>> {
    let height = (leaves.len() as f32).log2().ceil() as usize;
    assert!(height < MAX_HEIGHT, "Too many leaves.");

    let mut state = leaves
        .iter()
        .map(|v| hash_leaf(v.clone()))
        .collect::<Vec<_>>();

    // Pad the list of hashed leaves to a power of two
    let pad_len = 2_i32.pow(height as u32) - leaves.len() as i32;
    let mut pad = vec![b"\x00".to_vec(); pad_len as usize];
    state.append(&mut pad);

    // initialize a list that will contain the hashes in the proof
    let mut path = vec![];

    let mut level_pos = pos; // local copy of pos

    for _level in 0..height {
        let sibling = if level_pos % 2 == 0 {
            state[level_pos + 1].clone()
        } else {
            state[level_pos - 1].clone()
        };
        path.push(sibling);
        level_pos /= 2;

        // hash internal nodes in the tree
        state = (0..state.len())
            .step_by(2)
            .map(|i| hash_internal_node(state[i].clone(), state[i + 1].clone()))
            .collect::<Vec<_>>();
    }

    path
}

/// computes a root from the given leaf and Merkle proof.
pub fn compute_merkle_root_from_proof(proof: MerkleProof) -> Vec<u8> {
    let mut pos = proof.pos;
    let mut path = VecDeque::from(proof.path);
    let mut root = hash_leaf(proof.leaf);

    let (mut left, mut right) = (vec![], vec![]);
    while !path.is_empty() {
        if pos % 2 == 0 {
            left = root;
            if let Some(path) = path.pop_front() {
                right = path;
            }
        } else {
            if let Some(path) = path.pop_front() {
                left = path;
            }
            right = root;
        }
        root = hash_internal_node(left.clone(), right.clone());

        pos >>= 1;
    }

    root
}

#[test]
fn test_generate_merkle_tree() {
    let leaves = (0..1000)
        .map(|i| format!("data item {}", i).as_bytes().to_vec())
        .collect::<Vec<_>>();
    println!("I generated 1000 leaves for a Merkle tree of height 10.");

    // Generate proof for leaf #743
    let pos = 743;
    let path = gen_merkle_proof(leaves.clone(), pos);
    let proof = MerkleProof::new(leaves[pos].clone(), pos, path);

    // # write proof to file
    let _ = proof.write_proof(PROOFFILE.to_string());

    println!(
        "I generated a Merkle proof for leaf #{} in file {}\n",
        pos, PROOFFILE
    );

    let serde_proof = MerkleProofSerde::try_from(proof).unwrap();

    let serde_proof = serde_json::to_string_pretty(&serde_proof).unwrap();
    let mut fp = fs::File::create(SERDE_PROOF_FILE).unwrap();

    fp.write_all(serde_proof.as_bytes()).unwrap();

    println!("save serde merkle proof to file!");
}

#[test]
fn test_verify_merkle_proof() {
    let root = "1qIbsvuF6FrhNjMD4p06srUye6G4FfFINDDkNfKUpTs=";
    println!("\nHave hardcoded root of committed Merkle tree.");

    // # Read (leaf data, position of leaf, and proof) from file
    let proof = MerkleProof::read_proof(SERDE_PROOF_FILE.to_string()).unwrap();

    // # Verify that proof length is correct
    let height = proof.path.len();
    assert!(height < MAX_HEIGHT, "Proof is too long");

    // # Verify proof
    let computed_root = compute_merkle_root_from_proof(proof.clone());
    let base64_computed_root = base64::encode(computed_root);
    assert_eq!(root, base64_computed_root, "Verify failed");

    println!(
        "I verified the Merkle proof: leaf #{} in the committed tree is \"{}\".\n",
        proof.pos,
        String::from_utf8(proof.leaf).unwrap()
    );
}
