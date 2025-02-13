/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/meetups.json`.
 */
export type Meetups = {
  "address": "C14GzDxp9S1UfZk1BR1BPwHK1erFoPWkvjjFRtyf4B7L",
  "metadata": {
    "name": "meetups",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "createEvent",
      "discriminator": [
        49,
        219,
        29,
        203,
        22,
        98,
        100,
        87
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "eventsManager",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  110,
                  97,
                  103,
                  101,
                  114
                ]
              },
              {
                "kind": "arg",
                "path": "eventManagerId"
              }
            ]
          }
        },
        {
          "name": "identityProfile",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "hostProfile",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  104,
                  111,
                  115,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "eventsManager"
              },
              {
                "kind": "account",
                "path": "identityProfile"
              }
            ]
          }
        },
        {
          "name": "event",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  118,
                  101,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "eventsManager"
              },
              {
                "kind": "account",
                "path": "hostProfile"
              },
              {
                "kind": "arg",
                "path": "year"
              },
              {
                "kind": "arg",
                "path": "month"
              },
              {
                "kind": "arg",
                "path": "day"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "year",
          "type": "u16"
        },
        {
          "name": "month",
          "type": "u8"
        },
        {
          "name": "day",
          "type": "u8"
        }
      ]
    },
    {
      "name": "initEventManager",
      "discriminator": [
        238,
        92,
        141,
        123,
        185,
        233,
        187,
        107
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "eventManager",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  110,
                  97,
                  103,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "eventManagerState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "atlasMint",
          "type": "pubkey"
        },
        {
          "name": "polisMint",
          "type": "pubkey"
        },
        {
          "name": "usdcMint",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "initIdentityProfile",
      "discriminator": [
        23,
        80,
        230,
        226,
        25,
        157,
        72,
        239
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "identityProfile",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        }
      ]
    },
    {
      "name": "updateEventsManagerVaultFees",
      "discriminator": [
        246,
        140,
        144,
        56,
        232,
        1,
        143,
        156
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "eventManagerState"
          ]
        },
        {
          "name": "eventManagerState",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "daoVaultFee",
          "type": "u8"
        },
        {
          "name": "devVaultFee",
          "type": "u8"
        },
        {
          "name": "opsVaultFee",
          "type": "u8"
        },
        {
          "name": "hostProfileFee",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updateEventsManagerVaultOwners",
      "discriminator": [
        24,
        145,
        105,
        53,
        13,
        24,
        79,
        146
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "eventManagerState"
          ]
        },
        {
          "name": "eventManagerState",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "daoVaultOwner",
          "type": "pubkey"
        },
        {
          "name": "devVaultOwner",
          "type": "pubkey"
        },
        {
          "name": "opsVaultOwner",
          "type": "pubkey"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "eventEntry",
      "discriminator": [
        27,
        213,
        196,
        242,
        211,
        215,
        124,
        12
      ]
    },
    {
      "name": "eventsManager",
      "discriminator": [
        99,
        244,
        21,
        173,
        77,
        198,
        170,
        102
      ]
    },
    {
      "name": "eventsManagerState",
      "discriminator": [
        219,
        38,
        140,
        44,
        217,
        161,
        187,
        38
      ]
    },
    {
      "name": "hostProfile",
      "discriminator": [
        94,
        7,
        161,
        56,
        202,
        18,
        30,
        67
      ]
    },
    {
      "name": "identityProfile",
      "discriminator": [
        126,
        105,
        113,
        167,
        95,
        50,
        160,
        24
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "customError",
      "msg": "Custom error message"
    },
    {
      "code": 6001,
      "name": "eventNotPending",
      "msg": "Event is not pending"
    }
  ],
  "types": [
    {
      "name": "eventEntry",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "eventStatusType"
              }
            }
          },
          {
            "name": "host",
            "docs": [
              "Address of the HostProfile account"
            ],
            "type": "pubkey"
          },
          {
            "name": "name",
            "docs": [
              "Name of the event"
            ],
            "type": "string"
          },
          {
            "name": "year",
            "type": "u16"
          },
          {
            "name": "month",
            "type": "u8"
          },
          {
            "name": "day",
            "type": "u8"
          },
          {
            "name": "location",
            "type": "string"
          },
          {
            "name": "mappableAddress",
            "type": "string"
          },
          {
            "name": "startTimeAt",
            "type": "u64"
          },
          {
            "name": "endTimeAt",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "eventStatusType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "pending"
          },
          {
            "name": "open"
          },
          {
            "name": "closed"
          },
          {
            "name": "cancelled"
          },
          {
            "name": "completed"
          }
        ]
      }
    },
    {
      "name": "eventsManager",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "docs": [
              "Authority to make changes to the EventsManager"
            ],
            "type": "pubkey"
          },
          {
            "name": "mints",
            "type": {
              "defined": {
                "name": "mints"
              }
            }
          }
        ]
      }
    },
    {
      "name": "eventsManagerState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "docs": [
              "Authority to make changes to the EventsManagerState"
            ],
            "type": "pubkey"
          },
          {
            "name": "eventsManager",
            "docs": [
              "Address of the EventsManager account"
            ],
            "type": "pubkey"
          },
          {
            "name": "vaultFeeInfo",
            "type": {
              "defined": {
                "name": "vaultFeeInfo"
              }
            }
          },
          {
            "name": "vaultOwnerInfo",
            "type": {
              "defined": {
                "name": "vaultOwnerInfo"
              }
            }
          }
        ]
      }
    },
    {
      "name": "hostProfile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "eventsManager",
            "type": "pubkey"
          },
          {
            "name": "identityProfile",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "identityProfile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "docs": [
              "Pubkey of the user's wallet"
            ],
            "type": "pubkey"
          },
          {
            "name": "name",
            "docs": [
              "Name of the user"
            ],
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "mints",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "atlas",
            "docs": [
              "Mint address of Atlas token"
            ],
            "type": "pubkey"
          },
          {
            "name": "polis",
            "docs": [
              "Mint address of Polis token"
            ],
            "type": "pubkey"
          },
          {
            "name": "usdc",
            "docs": [
              "Mint address of USDC token"
            ],
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "vaultFeeInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "daoVaultFee",
            "type": "u8"
          },
          {
            "name": "devVaultFee",
            "type": "u8"
          },
          {
            "name": "opsVaultFee",
            "type": "u8"
          },
          {
            "name": "hostProfileFee",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "vaultOwnerInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "daoVaultOwner",
            "type": "pubkey"
          },
          {
            "name": "devVaultOwner",
            "type": "pubkey"
          },
          {
            "name": "opsVaultOwner",
            "type": "pubkey"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "seed",
      "type": "string",
      "value": "\"anchor\""
    }
  ]
};
