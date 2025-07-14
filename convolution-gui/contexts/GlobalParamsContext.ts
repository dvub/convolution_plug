import { createContext } from 'react';

export const GlobalParametersContext = createContext<{
	paramMap: string[];
} | null>(null);
