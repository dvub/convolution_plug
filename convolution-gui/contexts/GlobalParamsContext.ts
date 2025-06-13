import { Dispatch, SetStateAction } from 'react';
import { createContext } from 'react';

// TODO: FIX UNKNOWN
export const GlobalParametersContext = createContext<{
	parameters: { gain: number; dryWet: number };
	setParameters: Dispatch<SetStateAction<{ gain: number; dryWet: number }>>;
} | null>(null);
