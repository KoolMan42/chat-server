// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Operations = {
    queries: 
        { key: ["echo", string], result: string } | 
        { key: ["error"], result: string } | 
        { key: ["transformMe"], result: string } | 
        { key: ["version"], result: string },
    mutations: 
        { key: ["sendMsg", string], result: string },
    subscriptions: 
        { key: ["pings"], result: string }
};
