import { createContext } from 'react';

// TODO: fix this
export const GlobalParametersContext = createContext<{
	paramMap: string[];
} | null>(null);
