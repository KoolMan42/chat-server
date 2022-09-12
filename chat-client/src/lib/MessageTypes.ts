export interface User {
  username: string
  color: string
}


export interface ChatMessage  {
  user: User
  message: string
}
